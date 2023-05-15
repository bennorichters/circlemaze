use std::cmp::min;

use fraction::{ToPrimitive, Zero};

use super::components::{random_nr, Angle, CellState, CircleCoordinate, Direction, Grid};

pub fn build(outer_circle: u32, inner_slices: u32, min_dist: f64) -> CircularGrid {
    build_with_selector(
        outer_circle,
        inner_slices,
        min_dist,
        Box::new(random_option),
    )
}

type OptionSelector = dyn Fn(Vec<&CircleCoordinate>) -> CircleCoordinate;

fn random_option(options: Vec<&CircleCoordinate>) -> CircleCoordinate {
    options[random_nr(options.len())].to_owned()
}

pub fn build_with_selector(
    outer_circle: u32,
    inner_slices: u32,
    min_dist: f64,
    selector: Box<OptionSelector>,
) -> CircularGrid {
    let builder = CircularGridBuilder { inner_slices };

    let mut coords: Vec<Vec<CircleCoordinate>> = Vec::new();
    for circle in 0..=outer_circle {
        coords.push(builder.coords_on_circle(circle));
    }

    CircularGrid {
        coords,
        taken: Vec::new(),
        selector,
        inner_slices,
        min_dist,
    }
}

struct CircularGridBuilder {
    inner_slices: u32,
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

        CircleCoordinate {
            circle: coord.circle,
            angle: if next_numer == normalized_denom {
                Angle::from(0)
            } else {
                Angle::new(next_numer, normalized_denom)
            },
        }
    }
}

pub struct CircularGrid {
    coords: Vec<Vec<CircleCoordinate>>,
    taken: Vec<CircleCoordinate>,
    selector: Box<OptionSelector>,
    inner_slices: u32,
    min_dist: f64,
}

impl CircularGrid {
    fn remove_close_neighbours(&mut self, coord: &CircleCoordinate) {
        let circle = coord.circle;

        let index1 = neighbour_clockwise(&self.coords, coord).unwrap();
        let n1 = &self.coords[circle as usize][index1];
        if self.too_close(&coord.angle, &n1.angle, circle) {
            self.coords[circle as usize].remove(index1);
        }

        let index2 = neighbour_counter_clockwise(&self.coords, coord).unwrap();
        let n2 = &self.coords[circle as usize][index2];
        if self.too_close(&coord.angle, &n2.angle, circle) {
            self.coords[circle as usize].remove(index2);
        }
    }

    fn too_close(&self, a1: &Angle, a2: &Angle, circle: u32) -> bool {
        let dist = (a1 - a2).abs() * ((circle + 1) * self.inner_slices);
        dist.to_f64().unwrap() < self.min_dist
    }
}

impl Grid for CircularGrid {
    fn take_from_outer_circle(&mut self) -> (CircleCoordinate, CellState) {
        let options: Vec<&CircleCoordinate> = self.coords[self.coords.len() - 1].iter().collect();
        let coord = (self.selector)(options);
        let state = if self.taken.contains(&coord) {
            CellState::Taken
        } else {
            CellState::Free
        };
        if state == CellState::Free {
            self.taken.push(coord.to_owned());
        }
        self.remove_close_neighbours(&coord);
        (coord, state)
    }

    fn consume_outer_circle(&mut self) {
        // self.coords[self.outer_circle].iter().c
        todo!()
    }

    fn take_free(&mut self) -> Option<CircleCoordinate> {
        todo!()
    }

    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<(CircleCoordinate, CellState)> {
        todo!()
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

fn neighbour_out(coords: &[Vec<CircleCoordinate>], coord: &CircleCoordinate) -> Option<usize> {
    find(coords, coord.circle as usize + 1, &coord.angle)
}

fn neigbour_in(coords: &[Vec<CircleCoordinate>], coord: &CircleCoordinate) -> Option<usize> {
    if coord.circle == 0 {
        return None;
    }
    find(coords, coord.circle as usize - 1, &coord.angle)
}

fn neighbour_clockwise(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<usize> {
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
            Some(n)
        }
    } else {
        None
    }
}

fn neighbour_counter_clockwise(
    coords: &[Vec<CircleCoordinate>],
    coord: &CircleCoordinate,
) -> Option<usize> {
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
            Some(n)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod circular_grid_test {
    use crate::maze::{
        circular_grid::CircularGridBuilder,
        components::{Angle, CellState, CircleCoordinate, Direction, Grid},
        test_utils::helper_fns::create_coord,
    };

    use super::{build, build_with_selector};

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
    fn test_remove_close_neighbours_neighbour_on_max_dist() {
        let mut grid = build(5, 7, 0.5);
        let cloned = grid.coords[5].clone();
        grid.remove_close_neighbours(&create_coord(5, 0, 1));
        assert_eq!(cloned, grid.coords[5]);
    }

    #[test]
    fn test_remove_close_neighbours_one_to_remove_1_41_1_35() {
        let mut grid = build(5, 7, 0.5);
        grid.remove_close_neighbours(&create_coord(5, 1, 42));
        assert_eq!(69, grid.coords[5].len());
        assert!(!grid.coords[5].contains(&create_coord(5, 1, 35)));
    }

    #[test]
    fn test_remove_close_neighbours_max_dist_small_enough() {
        let mut grid = build(5, 7, 0.1);
        let cloned = grid.coords[5].clone();
        grid.remove_close_neighbours(&create_coord(5, 1, 42));
        assert_eq!(cloned, grid.coords[5]);
    }

    #[test]
    fn test_remove_close_neighbours_two_to_remove() {
        let mut grid = build(5, 7, 0.7);
        grid.remove_close_neighbours(&create_coord(5, 3, 35));
        assert_eq!(68, grid.coords[5].len());
        assert!(!grid.coords[5].contains(&create_coord(5, 1, 14)));
        assert!(!grid.coords[5].contains(&create_coord(5, 2, 21)));
    }

    #[test]
    fn test_remove_close_neighbours_one_to_remove_3_35_2_21() {
        let mut grid = build(5, 7, 0.5);
        grid.remove_close_neighbours(&create_coord(5, 3, 35));
        assert_eq!(69, grid.coords[5].len());
        assert!(!grid.coords[5].contains(&create_coord(5, 2, 21)));
    }

    #[test]
    fn test_take_from_outer_circle_options() {
        let check_and_select = |c: Vec<&CircleCoordinate>| {
            let values = vec![
                create_coord(2, 0, 1),
                create_coord(2, 1, 15),
                create_coord(2, 1, 10),
                create_coord(2, 2, 15),
                create_coord(2, 1, 5),
                create_coord(2, 4, 15),
                create_coord(2, 3, 10),
                create_coord(2, 1, 3),
                create_coord(2, 2, 5),
                create_coord(2, 7, 15),
                create_coord(2, 1, 2),
                create_coord(2, 8, 15),
                create_coord(2, 3, 5),
                create_coord(2, 2, 3),
                create_coord(2, 7, 10),
                create_coord(2, 11, 15),
                create_coord(2, 4, 5),
                create_coord(2, 13, 15),
                create_coord(2, 9, 10),
                create_coord(2, 14, 15),
            ];
            let expected: Vec<&CircleCoordinate> = values.iter().collect();

            assert_eq!(expected, c);

            c[0].to_owned()
        };
        let mut grid_zero_dist = build_with_selector(2, 5, 0., Box::new(check_and_select));
        _ = grid_zero_dist.take_from_outer_circle();
    }

    #[test]
    fn test_take_from_outer_circle_state() {
        let select_first = |c: Vec<&CircleCoordinate>| c[0].to_owned();
        let mut grid_zero_dist = build_with_selector(2, 5, 0., Box::new(select_first));

        let (_coord, state) = grid_zero_dist.take_from_outer_circle();
        assert_eq!(CellState::Free, state);

        let (_coord, state) = grid_zero_dist.take_from_outer_circle();
        assert_eq!(CellState::Taken, state);
    }

    #[test]
    fn test_take_from_outer_circle_neighbours_still_there() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_with_selector(5, 7, 0.1, Box::new(select_second));
        let (coord, _state) = grid.take_from_outer_circle();
        assert_eq!(create_coord(5, 1, 42), coord);
        assert_eq!(create_coord(5, 1, 35), grid.coords[5][2]);
    }

    #[test]
    fn test_take_from_outer_circle_neighbours_gone() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_with_selector(5, 7, 0.3, Box::new(select_second));
        let (coord, _state) = grid.take_from_outer_circle();
        assert_eq!(create_coord(5, 1, 42), coord);
        assert_eq!(create_coord(5, 1, 21), grid.coords[5][2]);
    }

    #[test]
    fn test_take_from_outer_circle_second_time_second_coordinate_is_still_there() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_with_selector(2, 5, 0., Box::new(select_second));
        let (coord, _state) = grid.take_from_outer_circle();
        assert_eq!(create_coord(2, 1, 15), coord);
        let (coord, _state) = grid.take_from_outer_circle();
        assert_eq!(create_coord(2, 1, 15), coord);
    }

    #[test]
    fn test_consume_outer_circle() {
        let mut grid = build(2, 5, 0.);
        grid.consume_outer_circle();
        assert!(grid.coords[2].iter().all(|c| grid.taken.contains(c)));
    }

    #[test]
    #[ignore]
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
            assert_eq!(
                pair.0,
                grid.take_neighbour(&pair.1, &Direction::Clockwise)
                    .unwrap()
                    .0
            );
            assert_eq!(
                pair.1,
                grid.take_neighbour(&pair.0, &Direction::CounterClockwise)
                    .unwrap()
                    .0
            );
        }
    }

    #[test]
    #[ignore]
    fn test_neighbours_on_line() {
        let mut grid = build(10, 7, 0.);
        let pair = pair(10, 0, 1, 9, 0, 1);
        assert_eq!(
            pair.0,
            grid.take_neighbour(&pair.1, &Direction::Out).unwrap().0
        );
        assert_eq!(
            pair.1,
            grid.take_neighbour(&pair.0, &Direction::In).unwrap().0
        );
    }

    #[test]
    #[ignore]
    fn test_coords_on_circle() {
        let builder = CircularGridBuilder { inner_slices: 7 };

        let coords = builder.coords_on_circle(0);
        assert_eq!(7, coords.len());

        let coords = builder.coords_on_circle(4);
        assert_eq!(56, coords.len());
    }

    #[test]
    #[ignore]
    fn test_take() {
        let mut grid = build(1, 4, 0.);
        assert_eq!(None, grid.take_free());
    }
}
