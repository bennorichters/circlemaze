use std::cmp::min;

use fraction::{ToPrimitive, Zero};

use super::components::{
    random_nr, Angle, CellState, CircleCoordinate, Direction, Distributor, Grid,
};

pub fn build(outer_circle: u32, inner_slices: u32, min_dist: f64) -> Box<dyn Grid> {
    Box::new(build_circular_grid(outer_circle, inner_slices, min_dist))
}

fn build_circular_grid(outer_circle: u32, inner_slices: u32, min_dist: f64) -> CircularGrid {
    CircularGrid {
        outer_circle,
        inner_slices,
        min_dist,
        coords: Vec::new(),
    }
}

struct CircularGrid {
    outer_circle: u32,
    inner_slices: u32,
    min_dist: f64,
    coords: Vec<Vec<CircleCoordinate>>,
}

type OptionSelector = dyn Fn(Vec<&CircleCoordinate>) -> CircleCoordinate;

fn random_option(options: Vec<&CircleCoordinate>) -> CircleCoordinate {
    options[random_nr(options.len())].to_owned()
}

impl CircularGrid {
    fn circular_dist(&mut self) -> CircularDist {
        self.circular_dist_with_selector(Box::new(random_option))
    }

    fn circular_dist_with_selector(&mut self, selector: Box<OptionSelector>) -> CircularDist {
        for circle in 0..=self.outer_circle {
            self.coords.push(self.coords_on_circle(circle));
        }

        CircularDist {
            coords: self.coords.clone(),
            taken: Vec::new(),
            selector,
            inner_slices: self.inner_slices,
            min_dist: self.min_dist,
        }
    }
}

impl Grid for CircularGrid {
    fn dist(&mut self) -> Box<dyn Distributor> {
        Box::new(self.circular_dist())
    }
}

impl CircularGrid {
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

pub struct CircularDist {
    coords: Vec<Vec<CircleCoordinate>>,
    taken: Vec<CircleCoordinate>,
    selector: Box<OptionSelector>,
    inner_slices: u32,
    min_dist: f64,
}

impl CircularDist {
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

    fn take(&mut self, coord: &CircleCoordinate) -> CellState {
        let state = if self.taken.contains(coord) {
            CellState::Taken
        } else {
            CellState::Free
        };
        if state == CellState::Free {
            self.taken.push(coord.to_owned());
        }
        self.remove_close_neighbours(coord);

        state
    }
}

impl Distributor for CircularDist {
    fn take_from_outer_circle(&mut self) -> (CircleCoordinate, CellState) {
        let options: Vec<&CircleCoordinate> = self.coords[self.coords.len() - 1].iter().collect();
        let coord = (self.selector)(options);
        let state = self.take(&coord);
        (coord, state)
    }

    fn consume_outer_circle(&mut self) {
        let outer_coords: Vec<CircleCoordinate> = self.coords[self.coords.len() - 1].clone();
        self.taken.extend(outer_coords);
    }

    fn take_free(&mut self) -> Option<CircleCoordinate> {
        let options: Vec<&CircleCoordinate> = self
            .coords
            .iter()
            .flatten()
            .filter(|c| !self.taken.contains(c))
            .collect();

        if options.is_empty() {
            None
        } else {
            let coord = (self.selector)(options);
            self.take(&coord);
            Some(coord)
        }
    }

    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<(CircleCoordinate, CellState)> {
        let (circle_index_diff, neigbour_index_option): (isize, Option<usize>) = match direction {
            Direction::Out => (1, neighbour_out(&self.coords, coord)),
            Direction::In => (-1, neigbour_in(&self.coords, coord)),
            Direction::Clockwise => (0, neighbour_clockwise(&self.coords, coord)),
            Direction::CounterClockwise => (0, neighbour_counter_clockwise(&self.coords, coord)),
        };

        if let Some(index_on_circle) = neigbour_index_option {
            let circle_index = (coord.circle as usize)
                .checked_add_signed(circle_index_diff)
                .unwrap();
            let coord = self.coords[circle_index][index_on_circle].to_owned();
            let state = self.take(&coord);
            Some((coord, state))
        } else {
            None
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
        circular_grid::build_circular_grid,
        components::{Angle, CellState, CircleCoordinate, Direction, Distributor, Grid},
        test_utils::helper_fns::create_coord,
    };

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
        let mut grid = build_circular_grid(5, 7, 0.5);
        let mut dist = grid.circular_dist();
        dist.remove_close_neighbours(&create_coord(5, 0, 1));
        assert_eq!(grid.coords[5], dist.coords[5]);
    }

    #[test]
    fn test_remove_close_neighbours_one_to_remove_1_41_1_35() {
        let mut grid = build_circular_grid(5, 7, 0.5);
        let mut dist = grid.circular_dist();
        dist.remove_close_neighbours(&create_coord(5, 1, 42));
        assert_eq!(69, dist.coords[5].len());
        assert!(!dist.coords[5].contains(&create_coord(5, 1, 35)));
    }

    #[test]
    fn test_remove_close_neighbours_max_dist_small_enough() {
        let mut grid = build_circular_grid(5, 7, 0.1);
        let mut dist = grid.circular_dist();
        dist.remove_close_neighbours(&create_coord(5, 1, 42));
        assert_eq!(grid.coords[5], dist.coords[5]);
    }

    #[test]
    fn test_remove_close_neighbours_two_to_remove() {
        let mut grid = build_circular_grid(5, 7, 0.7);
        let mut dist = grid.circular_dist();
        dist.remove_close_neighbours(&create_coord(5, 3, 35));
        assert_eq!(68, dist.coords[5].len());
        assert!(!dist.coords[5].contains(&create_coord(5, 1, 14)));
        assert!(!dist.coords[5].contains(&create_coord(5, 2, 21)));
    }

    #[test]
    fn test_remove_close_neighbours_one_to_remove_3_35_2_21() {
        let mut grid = build_circular_grid(5, 7, 0.5);
        let mut dist = grid.circular_dist();
        dist.remove_close_neighbours(&create_coord(5, 3, 35));
        assert_eq!(69, dist.coords[5].len());
        assert!(!dist.coords[5].contains(&create_coord(5, 2, 21)));
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
        let mut grid = build_circular_grid(2, 5, 0.);
        let mut dist_zero_dist = grid.circular_dist_with_selector(Box::new(check_and_select));
        dist_zero_dist.take_from_outer_circle();
    }

    #[test]
    fn test_take_from_outer_circle_state() {
        let select_first = |c: Vec<&CircleCoordinate>| c[0].to_owned();
        let mut grid = build_circular_grid(2, 5, 0.);
        let mut grid_zero_dist = grid.circular_dist_with_selector(Box::new(select_first));

        let (_coord, state) = grid_zero_dist.take_from_outer_circle();
        assert_eq!(CellState::Free, state);

        let (_coord, state) = grid_zero_dist.take_from_outer_circle();
        assert_eq!(CellState::Taken, state);
    }

    #[test]
    fn test_take_from_outer_circle_neighbours_still_there() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_circular_grid(5, 7, 0.1);
        let mut dist = grid.circular_dist_with_selector(Box::new(select_second));
        let (coord, _state) = dist.take_from_outer_circle();
        assert_eq!(create_coord(5, 1, 42), coord);
        assert_eq!(create_coord(5, 1, 35), dist.coords[5][2]);
    }

    #[test]
    fn test_take_from_outer_circle_neighbours_gone() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_circular_grid(5, 7, 0.3);
        let mut dist = grid.circular_dist_with_selector(Box::new(select_second));
        let (coord, _state) = dist.take_from_outer_circle();
        assert_eq!(create_coord(5, 1, 42), coord);
        assert_eq!(create_coord(5, 1, 21), dist.coords[5][2]);
    }

    #[test]
    fn test_take_from_outer_circle_second_time_second_coordinate_is_still_there() {
        let select_second = |c: Vec<&CircleCoordinate>| c[1].to_owned();
        let mut grid = build_circular_grid(2, 5, 0.);
        let mut dist = grid.circular_dist_with_selector(Box::new(select_second));
        let (coord, _state) = dist.take_from_outer_circle();
        assert_eq!(create_coord(2, 1, 15), coord);
        let (coord, _state) = dist.take_from_outer_circle();
        assert_eq!(create_coord(2, 1, 15), coord);
    }

    #[test]
    fn test_consume_outer_circle() {
        let mut grid = build_circular_grid(2, 5, 0.);
        let mut dist = grid.circular_dist();
        dist.consume_outer_circle();
        assert!(dist.coords[2].iter().all(|c| dist.taken.contains(c)));
    }

    #[test]
    fn test_neighbours_on_arc() {
        let mut grid = build_circular_grid(10, 7, 0.);
        let mut dist = grid.circular_dist();

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
                dist.take_neighbour(&pair.1, &Direction::Clockwise)
                    .unwrap()
                    .0
            );
            assert_eq!(
                pair.1,
                dist.take_neighbour(&pair.0, &Direction::CounterClockwise)
                    .unwrap()
                    .0
            );
        }
    }

    #[test]
    fn test_neighbours_on_line() {
        let mut grid = build_circular_grid(10, 7, 0.);
        let mut dist = grid.circular_dist();
        let pair = pair(10, 0, 1, 9, 0, 1);
        assert_eq!(
            pair.0,
            dist.take_neighbour(&pair.1, &Direction::Out).unwrap().0
        );
        assert_eq!(
            pair.1,
            dist.take_neighbour(&pair.0, &Direction::In).unwrap().0
        );
    }

    #[test]
    fn test_coords_on_circle() {
        let grid = build_circular_grid(4, 7, 0.);

        let coords = grid.coords_on_circle(0);
        assert_eq!(7, coords.len());

        let coords = grid.coords_on_circle(4);
        assert_eq!(56, coords.len());
    }

    #[test]
    fn test_take() {
        let select_first = |c: Vec<&CircleCoordinate>| c[0].to_owned();
        let mut grid = build_circular_grid(1, 4, 0.);
        let mut dist = grid.circular_dist_with_selector(Box::new(select_first));

        for _ in 0..12 {
            assert!(dist.take_free().is_some());
        }

        assert!(dist.take_free().is_none());
    }
}
