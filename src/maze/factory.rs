use std::cmp::min;

use fraction::Zero;

use super::components::{Angle, Border, BorderType, CircleCoordinate};

pub fn create_maze(circles: u32, inner_slices: u32, min_dist: f64) -> Vec<Border> {
    let mut maze = Maze {
        outer_circle: circles,
        inner_slices,
        min_dist,
        borders: Vec::new(),
    };
    let mut open_coords = maze.all_coords();
    maze.close_outer_circle();

    while !open_coords.is_empty() {
        let coord = &open_coords[random_index(open_coords.len())];
        let path_coords = maze.create_path(coord, &open_coords);
        open_coords.retain(|e| !path_coords.contains(e));
    }
    maze.borders
}

struct Maze {
    outer_circle: u32,
    inner_slices: u32,
    min_dist: f64,
    borders: Vec<Border>,
}

impl Maze {
    fn close_outer_circle(&mut self) {
        self.borders.push(Border {
            start: CircleCoordinate {
                circle: self.outer_circle,
                angle: Angle::from(0),
            },
            end: CircleCoordinate {
                circle: self.outer_circle,
                angle: Angle::from(0),
            },
        });
    }

    fn create_path(
        &mut self,
        start_coord: &CircleCoordinate,
        open_coords: &[CircleCoordinate],
    ) -> Vec<CircleCoordinate> {
        let mut visited: Vec<CircleCoordinate> = vec![start_coord.to_owned()];
        let mut options: Vec<(CircleCoordinate, Direction)> = Vec::new();
        let mut coord = start_coord.to_owned();
        while open_coords.contains(&coord) {
            add_options(&mut options, &coord);
            let (from_coord, to_coord, direction) = self.next(&mut options, &visited);
            coord = to_coord.to_owned();

            visited.push(to_coord.to_owned());
            let (merge_start, merge_end, border_type) = match direction {
                Direction::Out => (from_coord, to_coord, BorderType::Line),
                Direction::In => (to_coord, from_coord, BorderType::Line),
                Direction::Clockwise => (from_coord, to_coord, BorderType::Arc),
                Direction::CounterClockwise => (to_coord, from_coord, BorderType::Arc),
            };
            self.merge_borders(merge_start, merge_end, border_type);
        }

        visited
    }

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

    fn merge_borders(
        &mut self,
        start: CircleCoordinate,
        end: CircleCoordinate,
        border_type: BorderType,
    ) {
        let mut merged_start = start;
        let mut merged_end = end;

        if let Some(before_index) = self.find_merge_start(&merged_start, &border_type) {
            let before = self.borders.remove(before_index);
            merged_start = before.start;
        }

        if let Some(after_index) = self.find_merge_end(&merged_end, &border_type) {
            let after = self.borders.remove(after_index);
            merged_end = after.end;
        }

        self.borders.push(Border {
            start: merged_start,
            end: merged_end,
        });
    }

    fn find_merge_start(
        &self,
        from_coord: &CircleCoordinate,
        border_type: &BorderType,
    ) -> Option<usize> {
        self.borders
            .iter()
            .position(|b| &b.border_type() == border_type && &b.end == from_coord)
    }

    fn find_merge_end(
        &self,
        to_coord: &CircleCoordinate,
        border_type: &BorderType,
    ) -> Option<usize> {
        self.borders
            .iter()
            .position(|b| &b.border_type() == border_type && &b.start == to_coord)
    }

    fn slices_on_circle(&self, circle: u32) -> u32 {
        (circle + 1) * self.inner_slices
    }

    fn all_coords(&self) -> Vec<CircleCoordinate> {
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
}

fn add_options(options: &mut Vec<(CircleCoordinate, Direction)>, coord: &CircleCoordinate) {
    options.push((coord.to_owned(), Direction::In));
    options.push((coord.to_owned(), Direction::Out));
    options.push((coord.to_owned(), Direction::Clockwise));
    options.push((coord.to_owned(), Direction::CounterClockwise));
}

fn random_index(length: usize) -> usize {
    (rand::random::<f32>() * length as f32).floor() as usize
}

#[derive(Debug)]
enum Direction {
    Out,
    In,
    Clockwise,
    CounterClockwise,
}

#[cfg(test)]
mod factory_tests {
    use crate::maze::{
        components::{Angle, CircleCoordinate},
        factory::Maze,
    };

    fn pairs() -> Vec<(CircleCoordinate, CircleCoordinate)> {
        vec![
            (
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(1_u32, 7_u32),
                },
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::from(0),
                },
            ),
            (
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(2_u32, 7_u32),
                },
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(1_u32, 7_u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::from(0),
                },
                CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(6_u32, 7_u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, (5 * 7) as u32),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::from(0),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, (4 * 7) as u32),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, (5 * 7) as u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(2_u32, (5 * 7) as u32),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, (4 * 7) as u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, 2_u32),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(17_u32, (5 * 7) as u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(18_u32, (5 * 7) as u32),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(1_u32, 2_u32),
                },
            ),
            (
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::from(0),
                },
                CircleCoordinate {
                    circle: 4,
                    angle: Angle::new(34_u32, (5 * 7) as u32),
                },
            ),
        ]
    }

    #[test]
    fn test_next_coord_on_circle() {
        let maze = Maze {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
            borders: vec![],
        };

        for pair in pairs() {
            println!("{:?}", pair);
            assert_eq!(pair.0, maze.next_coord_on_circle(&pair.1));
        }
    }

    #[test]
    fn test_prev_coord_on_circle() {
        let maze = Maze {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
            borders: vec![],
        };

        for pair in pairs() {
            assert_eq!(pair.1, maze.prev_coord_on_circle(&pair.0));
        }
    }

    #[test]
    fn test_coords_on_circle() {
        let maze = Maze {
            outer_circle: 10,
            inner_slices: 7,
            min_dist: 0.,
            borders: vec![],
        };

        let coords = maze.coords_on_circle(0);
        assert_eq!(7, coords.len());

        let coords = maze.coords_on_circle(4);
        assert_eq!(56, coords.len());
    }
}
