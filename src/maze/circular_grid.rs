use std::cmp::min;

use fraction::{ToPrimitive, Zero};

use super::components::{
    random_nr, Angle, CellState, CircleCoordinate, Direction, Distributor, Grid,
};

pub fn build_grid(outer_circle: u32, inner_slices: u32, min_dist: f64) -> impl Grid {
    let mut result = CircularGrid {
        outer_circle,
        inner_slices,
        min_dist,
        coords: Vec::new(),
    };

    result.fill_coords();

    result
}

type OptionSelector<'a> = dyn Fn(&[&'a CircleCoordinate]) -> &'a CircleCoordinate;

fn random_option<'a>(options: &[&'a CircleCoordinate]) -> &'a CircleCoordinate {
    options[random_nr(options.len())]
}

struct CircularGrid {
    outer_circle: u32,
    inner_slices: u32,
    min_dist: f64,
    coords: Vec<Vec<CircleCoordinate>>,
}

impl CircularGrid {
    fn fill_coords(&mut self) {
        for circle in 0..=self.outer_circle {
            self.coords.push(self.coords_on_circle(circle));
        }
    }

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

impl Grid for CircularGrid {
    type Dist<'a> = CircularDist<'a> where Self: 'a;

    fn dist(&self) -> Self::Dist<'_> {
        let dist_coords: Vec<Vec<&CircleCoordinate>> = Vec::new();
        for circle in 0..=self.outer_circle {
            let mut ref_coordds_on_circle: Vec<&CircleCoordinate> = Vec::new();
            for coords_on_circle in &self.coords[circle as usize] {
                ref_coordds_on_circle.push(coords_on_circle);
            }
        }

        CircularDist {
            coords: dist_coords,
            taken: Vec::new(),
            selector: Box::new(random_option),
            inner_slices: self.inner_slices,
            min_dist: self.min_dist,
        }
    }
}

pub struct CircularDist<'a> {
    coords: Vec<Vec<&'a CircleCoordinate>>,
    taken: Vec<&'a CircleCoordinate>,
    selector: Box<OptionSelector<'a>>,
    inner_slices: u32,
    min_dist: f64,
}

impl<'a> CircularDist<'a> {
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

    fn take(&mut self, coord: &'a CircleCoordinate) -> CellState {
        let state = if self.taken.contains(&coord) {
            CellState::Taken
        } else {
            CellState::Free
        };
        if state == CellState::Free {
            self.taken.push(coord);
        }
        self.remove_close_neighbours(coord);

        state
    }
}

impl Distributor for CircularDist<'_> {
    fn take_from_outer_circle(&mut self) -> (&CircleCoordinate, CellState) {
        let options: &Vec<&CircleCoordinate> = &self.coords[self.coords.len() - 1];
        let coord = (self.selector)(options);
        let state = self.take(coord);
        (coord, state)
    }

    fn consume_outer_circle(&mut self) {
        for c in &self.coords[self.coords.len() - 1] {
            self.taken.push(c);
        }
    }

    fn take_free(&mut self) -> Option<&CircleCoordinate> {
        let options: Vec<&CircleCoordinate> = self
            .coords
            .iter()
            .flatten()
            .filter(|c| !self.taken.contains(c))
            .copied()
            .collect();

        if options.is_empty() {
            None
        } else {
            let coord = (self.selector)(&options);
            self.take(coord);
            Some(coord)
        }
    }

    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<(&CircleCoordinate, CellState)> {
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
            let coord = self.coords[circle_index][index_on_circle];
            let state = self.take(coord);
            Some((coord, state))
        } else {
            None
        }
    }
}

fn find(coords: &[Vec<&CircleCoordinate>], circle: usize, angle: &Angle) -> Option<usize> {
    if circle < coords.len() {
        let coords_on_circle = &coords[circle];
        coords_on_circle
            .binary_search(&&CircleCoordinate {
                circle: circle as u32,
                angle: *angle,
            })
            .ok()
    } else {
        None
    }
}

fn neighbour_out(coords: &[Vec<&CircleCoordinate>], coord: &CircleCoordinate) -> Option<usize> {
    find(coords, coord.circle as usize + 1, &coord.angle)
}

fn neigbour_in(coords: &[Vec<&CircleCoordinate>], coord: &CircleCoordinate) -> Option<usize> {
    if coord.circle == 0 {
        return None;
    }
    find(coords, coord.circle as usize - 1, &coord.angle)
}

fn neighbour_clockwise(
    coords: &[Vec<&CircleCoordinate>],
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
    coords: &[Vec<&CircleCoordinate>],
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
