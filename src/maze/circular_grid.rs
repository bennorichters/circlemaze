use std::cmp::min;

use fraction::Zero;

use super::{
    components::{Angle, CircleCoordinate},
    factory::{Direction, Grid},
};

pub struct CircularGrid {
    pub outer_circle: u32,
    pub inner_slices: u32,
    pub min_dist: f64,
}

impl CircularGrid {
    pub fn all_coords(&self) -> Vec<CircleCoordinate> {
        let mut result: Vec<CircleCoordinate> = Vec::new();

        for circle in 0..self.outer_circle {
            result.extend(self.coords_on_circle(circle));
        }

        result
    }

    fn coords_on_circle(&self, circle: u32) -> Vec<CircleCoordinate> {
        let mut result: Vec<CircleCoordinate> = Vec::new();
        let mut coord = CircleCoordinate {
            circle,
            angle: Angle::from(0),
        };

        loop {
            result.push(coord.to_owned());
            coord = self.next_coord_on_circle(&coord);
            if coord.angle.is_zero() {
                break;
            }
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

        if c.is_on_grid(self.inner_slices, self.min_dist) {
            c
        } else {
            self.next_coord_on_circle(&c)
        }
    }

    fn prev_coord_on_circle(&self, coord: &CircleCoordinate) -> CircleCoordinate {
        if *coord.angle.numer().unwrap() == 0 {
            let steps = self.slices_on_circle(coord.circle);
            return CircleCoordinate {
                circle: coord.circle,
                angle: Angle::new(steps - 1, steps),
            };
        }

        if coord.circle == 0 {
            return CircleCoordinate {
                circle: 0,
                angle: coord.angle - Angle::new(1_u32, self.inner_slices),
            };
        }

        let n = coord.angle.numer().unwrap();
        let d = coord.angle.denom().unwrap();

        let normalized_denom = coord.circle * (coord.circle + 1) * self.inner_slices;
        let normalized_numer = n * (normalized_denom / d);

        let diff1 = normalized_numer % coord.circle;
        let diff2 = normalized_numer % (coord.circle + 1);

        let diff1 = if diff1 == 0 { coord.circle } else { diff1 };
        let diff2 = if diff2 == 0 { coord.circle + 1 } else { diff2 };

        let next_numer = normalized_numer - min(diff1, diff2);

        let c = CircleCoordinate {
            circle: coord.circle,
            angle: Angle::new(next_numer, normalized_denom),
        };
        if c.is_on_grid(self.inner_slices, self.min_dist) {
            c
        } else {
            self.prev_coord_on_circle(&c)
        }
    }

    fn slices_on_circle(&self, circle: u32) -> u32 {
        (circle + 1) * self.inner_slices
    }

    fn neighbour(
        &self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<CircleCoordinate> {
        match direction {
            Direction::Out => {
                if coord.circle < self.outer_circle {
                    let candidate = CircleCoordinate {
                        circle: coord.circle + 1,
                        angle: coord.angle.to_owned(),
                    };

                    if candidate.is_on_grid(self.inner_slices, self.min_dist) {
                        Some(candidate)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Direction::In => {
                if coord.circle > 0 {
                    let candidate = CircleCoordinate {
                        circle: coord.circle - 1,
                        angle: coord.angle.to_owned(),
                    };
                    if candidate.is_on_grid(self.inner_slices, self.min_dist) {
                        Some(candidate)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Direction::Clockwise => Some(self.next_coord_on_circle(coord)),
            Direction::CounterClockwise => Some(self.prev_coord_on_circle(coord)),
        }
    }
}

impl Grid for CircularGrid {
    fn next(
        &self,
        options: &mut Vec<(CircleCoordinate, Direction)>,
        current_path: &[CircleCoordinate],
    ) -> (CircleCoordinate, CircleCoordinate, Direction) {
        while !options.is_empty() {
            let (candidate_start, candidate_direction) =
                options.remove(random_index(options.len()));
            let neighbour = self.neighbour(&candidate_start, &candidate_direction);
            if let Some(end) = neighbour {
                if !current_path.contains(&end) {
                    return (candidate_start, end, candidate_direction);
                }
            }
        }

        panic!();
    }
}

fn random_index(length: usize) -> usize {
    (rand::random::<f32>() * length as f32).floor() as usize
}

#[cfg(test)]
mod factory_tests {
    use crate::maze::components::{Angle, CircleCoordinate};

    use super::CircularGrid;

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

    fn pairs() -> Vec<(CircleCoordinate, CircleCoordinate)> {
        vec![
            pair(0, 1, 7, 0, 0, 1),
            pair(0, 2, 7, 0, 1, 7),
            pair(0, 0, 1, 0, 6, 7),
            pair(4, 1, 35, 4, 0, 1),
            pair(4, 1, 28, 4, 1, 35),
            pair(4, 2, 35, 4, 1, 28),
            pair(4, 1, 2, 4, 17, 35),
            pair(4, 18, 35, 4, 1, 2),
            pair(4, 0, 1, 4, 34, 35),
        ]
    }

    #[test]
    fn test_next_coord_on_circle() {
        let maze = CircularGrid {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
        };

        for pair in pairs() {
            println!("{:?}", pair);
            assert_eq!(pair.0, maze.next_coord_on_circle(&pair.1));
        }
    }

    #[test]
    fn test_prev_coord_on_circle() {
        let maze = CircularGrid {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
        };

        for pair in pairs() {
            assert_eq!(pair.1, maze.prev_coord_on_circle(&pair.0));
        }
    }

    #[test]
    fn test_coords_on_circle() {
        let maze = CircularGrid {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
        };

        let coords = maze.coords_on_circle(0);
        assert_eq!(7, coords.len());

        let coords = maze.coords_on_circle(4);
        assert_eq!(56, coords.len());
    }
}
