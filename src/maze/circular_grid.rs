use std::cmp::min;

use fraction::{ToPrimitive, Zero};

use super::components::{random_nr, Angle, Border, CircleCoordinate, Direction, Grid};

pub fn build(outer_circle: u32, inner_slices: u32, min_dist: f64) -> CircularGrid {
    let builder = CircularGridBuilder {
        inner_slices,
        min_dist,
    };

    let mut coords: Vec<Vec<CircleCoordinate>> = Vec::new();
    for circle in 0..=outer_circle {
        coords.push(builder.coords_on_circle(circle));
    }

    CircularGrid { coords }
}

struct CircularGridBuilder {
    inner_slices: u32,
    min_dist: f64,
}

impl CircularGridBuilder {
    fn coords_on_circle(&self, circle: u32) -> Vec<CircleCoordinate> {
        let mut result: Vec<CircleCoordinate> = Vec::new();
        let mut coord = CircleCoordinate {
            circle,
            angle: Angle::from(0),
        };

        loop {
            result.push(coord.to_owned());
            let angle = self.next_coord_on_circle(&coord);
            if angle.angle.is_zero() {
                break;
            }
            coord = angle;
        }

        result
    }

    fn next_coord_on_circle(&self, coord: &CircleCoordinate) -> CircleCoordinate {
        if coord.circle == 0 {
            return CircleCoordinate {
                circle: 0,
                angle: if coord.angle == Angle::new(self.inner_slices - 1, self.inner_slices) {
                    Angle::from(0)
                } else {
                    coord.angle + Angle::new(1_u32, self.inner_slices)
                },
            };
        }

        let n = coord.angle.numer().unwrap();
        let d = coord.angle.denom().unwrap();

        let normalized_denom = coord.circle * (coord.circle + 1) * self.inner_slices;
        let normalized_numer = n * (normalized_denom / d);

        let diff1 = coord.circle - (normalized_numer % coord.circle);
        let diff2 = (coord.circle + 1) - (normalized_numer % (coord.circle + 1));

        let next_numer = normalized_numer + min(diff1, diff2);

        let c = CircleCoordinate {
            circle: coord.circle,
            angle: if next_numer == normalized_denom {
                Angle::from(0)
            } else {
                Angle::new(next_numer, normalized_denom)
            },
        };

        if self.is_on_grid(&c) {
            c
        } else {
            self.next_coord_on_circle(&c)
        }
    }

    pub fn is_on_grid(&self, coord: &CircleCoordinate) -> bool {
        let denom = coord.angle.denom().unwrap();
        if ((coord.circle + 1) * self.inner_slices) % denom == 0 {
            return true;
        }

        if coord.circle == 0 {
            return false;
        }

        if (coord.circle * self.inner_slices) % denom != 0 {
            return false;
        }

        let section = (coord.angle * self.inner_slices).floor().to_u32().unwrap();
        let angle_in_section = coord.angle - Angle::new(section, self.inner_slices);
        let relative = angle_in_section * self.inner_slices;

        let dist = relative.to_f64().unwrap();
        self.min_dist <= dist && dist <= (1. - self.min_dist)
    }
}

pub struct CircularGrid {
    coords: Vec<Vec<CircleCoordinate>>,
}

impl CircularGrid {
    fn coords_not_on_borders(&self, borders: &[Border]) -> Vec<CircleCoordinate> {
        let cs = self.all_coords();
        cs.iter()
            .filter(|&c| !borders.iter().any(|b| b.contains(c)))
            .cloned()
            .collect()
    }

    fn all_coords(&self) -> Vec<CircleCoordinate> {
        let mut result: Vec<CircleCoordinate> = Vec::new();

        let outer = self.coords.len() - 1;
        for circle in 0..=outer {
            let coords_on_circle = self.coords[circle].clone();
            result.extend(coords_on_circle);
        }

        result
    }
}

impl Grid for CircularGrid {
    fn take_from_outer_circle(&mut self) -> CircleCoordinate {
        todo!()
    }

    fn consume_outer_circle(&mut self) {
        todo!()
    }

    fn take_free(&mut self, borders: &[Border]) -> Option<CircleCoordinate> {
        let options = self.coords_not_on_borders(borders);

        if options.is_empty() {
            None
        } else {
            Some(options[random_nr(options.len())].clone())
        }
    }

    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<CircleCoordinate> {
        match direction {
            Direction::Out => neighbour_out(&self.coords, coord),
            Direction::In => neigbour_in(&self.coords, coord),
            Direction::Clockwise => neighbour_clockwise(&self.coords, coord),
            Direction::CounterClockwise => neighbour_counter_clockwise(&self.coords, coord),
        }
    }
}

fn find(coords: &[Vec<CircleCoordinate>], circle: usize, angle: &Angle) -> Option<usize> {
    if circle < coords.len() {
        let coords_on_circle = &coords[circle];
        coords_on_circle
            .binary_search(&CircleCoordinate {
                circle: circle as u32,
                angle: *angle,
            })
            .ok()
    } else {
        None
    }
}

fn neighbour_out(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<CircleCoordinate> {
    let index_option = find(coords, coord.circle as usize + 1, &coord.angle);
    if index_option.is_some() {
        Some(CircleCoordinate {
            circle: coord.circle + 1,
            angle: coord.angle.to_owned(),
        })
    } else {
        None
    }
}

fn neigbour_in(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<CircleCoordinate> {
    if coord.circle == 0 {
        return None;
    }
    let index_option = find(coords, coord.circle as usize - 1, &coord.angle);
    if index_option.is_some() {
        Some(CircleCoordinate {
            circle: coord.circle - 1,
            angle: coord.angle.to_owned(),
        })
    } else {
        None
    }
}

fn neighbour_clockwise(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<CircleCoordinate> {
    let index_option = find(coords, coord.circle as usize, &coord.angle);
    if let Some(index) = index_option {
        let coords_on_circle = &coords[coord.circle as usize];
        if coords_on_circle.len() == 1 {
            None
        } else {
            let n = if index == coords_on_circle.len() - 1 {
                0
            } else {
                index + 1
            };
            Some(coords_on_circle[n].to_owned())
        }
    } else {
        None
    }
}

fn neighbour_counter_clockwise(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<CircleCoordinate> {
    let index_option = find(coords, coord.circle as usize, &coord.angle);
    if let Some(index) = index_option {
        let coords_on_circle = &coords[coord.circle as usize];
        if coords_on_circle.len() == 1 {
            None
        } else {
            let n = if index == 0 {
                coords_on_circle.len() - 1
            } else {
                index - 1
            };
            Some(coords_on_circle[n].to_owned())
        }
    } else {
        None
    }
}

#[cfg(test)]
mod circular_grid_test {
    use crate::maze::{
        circular_grid::CircularGridBuilder,
        components::{Angle, Border, CircleCoordinate, Direction, Grid},
        test_utils::helper_fns::{create_border, create_coord},
    };

    use super::build;

    fn pair(
        circle1: u32,
        numer1: u32,
        denom1: u32,
        circle2: u32,
        numer2: u32,
        denom2: u32,
    ) -> (CircleCoordinate, CircleCoordinate) {
        (
            CircleCoordinate {
                circle: circle1,
                angle: Angle::new(numer1, denom1),
            },
            CircleCoordinate {
                circle: circle2,
                angle: Angle::new(numer2, denom2),
            },
        )
    }

    #[test]
    fn test_neighbours_on_arc() {
        let mut grid = build(10, 7, 0.);

        for pair in vec![
            pair(0, 1, 7, 0, 0, 1),
            pair(0, 2, 7, 0, 1, 7),
            pair(0, 0, 1, 0, 6, 7),
            pair(4, 1, 35, 4, 0, 1),
            pair(4, 1, 28, 4, 1, 35),
            pair(4, 2, 35, 4, 1, 28),
            pair(4, 1, 2, 4, 17, 35),
            pair(4, 18, 35, 4, 1, 2),
            pair(4, 0, 1, 4, 34, 35),
        ] {
            println!("{:?}", pair);
            assert_eq!(
                pair.0,
                grid.take_neighbour(&pair.1, &Direction::Clockwise).unwrap()
            );
            assert_eq!(
                pair.1,
                grid.take_neighbour(&pair.0, &Direction::CounterClockwise)
                    .unwrap()
            );
        }
    }

    #[test]
    fn test_neighbours_on_line() {
        let mut grid = build(10, 7, 0.);
        let pair = pair(10, 0, 1, 9, 0, 1);
        assert_eq!(
            pair.0,
            grid.take_neighbour(&pair.1, &Direction::Out).unwrap()
        );
        assert_eq!(
            pair.1,
            grid.take_neighbour(&pair.0, &Direction::In).unwrap()
        );
    }

    #[test]
    fn test_coords_on_circle() {
        let builder = CircularGridBuilder {
            inner_slices: 7,
            min_dist: 0.,
        };

        let coords = builder.coords_on_circle(0);
        assert_eq!(7, coords.len());

        let coords = builder.coords_on_circle(4);
        assert_eq!(56, coords.len());
    }

    fn on_grid_pass(circle: u32, numer: u32, denom: u32, slices: u32, min_dist: f64) {
        let builder = CircularGridBuilder {
            inner_slices: slices,
            min_dist,
        };
        let coord = CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom),
        };
        assert!(builder.is_on_grid(&coord));
    }

    fn on_grid_fail(circle: u32, numer: u32, denom: u32, slices: u32, min_dist: f64) {
        let builder = CircularGridBuilder {
            inner_slices: slices,
            min_dist,
        };
        let coord = CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom),
        };
        assert!(!builder.is_on_grid(&coord));
    }

    #[test]
    fn test_is_on_grid() {
        on_grid_pass(0, 0, 1, 3, 1.);
        on_grid_pass(0, 0, 1, 3, 0.);
        on_grid_pass(0, 1, 3, 3, 1.);
        on_grid_pass(0, 1, 3, 3, 0.);
        on_grid_pass(0, 2, 3, 3, 1.);
        on_grid_pass(0, 2, 3, 3, 0.);
        on_grid_pass(1, 1, 6, 3, 1.);
        on_grid_pass(1, 1, 6, 3, 0.);
        on_grid_pass(1, 5, 6, 3, 0.);
        on_grid_pass(1, 1, 3, 3, 0.);

        on_grid_pass(1, 1, 3, 3, 0.33);
        on_grid_pass(1, 2, 3, 3, 0.33);

        on_grid_fail(0, 1, 4, 3, 1.);

        on_grid_pass(4, 1, 28, 7, 0.);
        on_grid_fail(4, 1, 28, 7, 0.3);
        on_grid_pass(5, 1, 35, 7, 0.);
        on_grid_fail(5, 1, 35, 7, 0.3);
        on_grid_pass(5, 1, 35, 7, 0.);
        on_grid_fail(5, 1, 35, 7, 0.3);
        on_grid_fail(5, 4, 35, 7, 0.3);
        on_grid_pass(5, 6, 35, 7, 0.);
        on_grid_fail(5, 6, 35, 7, 0.3);

        on_grid_pass(3, 11, 21, 7, 0.3);
        on_grid_fail(3, 11, 21, 7, 0.34);
    }

    #[test]
    fn test_coords_not_on_border_no_borders() {
        let grid = build(1, 4, 0.);
        let borders: Vec<Border> = vec![];
        let coords = grid.coords_not_on_borders(&borders);

        assert_eq!(12, coords.len());

        assert!(coords.contains(&create_coord(0, 0, 1)));
        assert!(coords.contains(&create_coord(0, 1, 4)));
        assert!(coords.contains(&create_coord(0, 1, 2)));
        assert!(coords.contains(&create_coord(0, 3, 4)));

        assert!(coords.contains(&create_coord(1, 0, 1)));
        assert!(coords.contains(&create_coord(1, 1, 8)));
        assert!(coords.contains(&create_coord(1, 1, 4)));
        assert!(coords.contains(&create_coord(1, 3, 8)));
        assert!(coords.contains(&create_coord(1, 1, 2)));
        assert!(coords.contains(&create_coord(1, 5, 8)));
        assert!(coords.contains(&create_coord(1, 3, 4)));
        assert!(coords.contains(&create_coord(1, 7, 8)));
    }

    #[test]
    fn test_coords_not_on_border_border_outer_circle() {
        let grid = build(1, 4, 0.);
        let borders: Vec<Border> = vec![create_border(1, 0, 1, 1, 0, 1)];
        let coords = grid.coords_not_on_borders(&borders);

        assert_eq!(4, coords.len());

        assert!(coords.contains(&create_coord(0, 0, 1)));
        assert!(coords.contains(&create_coord(0, 1, 4)));
        assert!(coords.contains(&create_coord(0, 1, 2)));
        assert!(coords.contains(&create_coord(0, 3, 4)));
    }

    #[test]
    fn test_coords_not_on_border_line_border() {
        let grid = build(1, 4, 0.);
        let borders: Vec<Border> = vec![create_border(0, 0, 1, 1, 0, 1)];
        let coords = grid.coords_not_on_borders(&borders);

        assert_eq!(10, coords.len());

        assert!(coords.contains(&create_coord(0, 1, 4)));
        assert!(coords.contains(&create_coord(0, 1, 2)));
        assert!(coords.contains(&create_coord(0, 3, 4)));

        assert!(coords.contains(&create_coord(1, 1, 8)));
        assert!(coords.contains(&create_coord(1, 1, 4)));
        assert!(coords.contains(&create_coord(1, 3, 8)));
        assert!(coords.contains(&create_coord(1, 1, 2)));
        assert!(coords.contains(&create_coord(1, 5, 8)));
        assert!(coords.contains(&create_coord(1, 3, 4)));
        assert!(coords.contains(&create_coord(1, 7, 8)));
    }

    #[test]
    fn test_coords_not_on_border_all_occupied() {
        let grid = build(1, 4, 0.);
        let borders: Vec<Border> = vec![
            create_border(1, 0, 1, 1, 0, 1),
            create_border(0, 0, 1, 0, 0, 1),
        ];
        let coords = grid.coords_not_on_borders(&borders);

        assert!(coords.is_empty());
    }

    #[test]
    fn test_take() {
        let mut grid = build(1, 4, 0.);
        let borders: Vec<Border> = vec![
            create_border(1, 0, 1, 1, 0, 1),
            create_border(0, 0, 1, 0, 0, 1),
        ];

        assert_eq!(None, grid.take_free(&borders));
    }
}
