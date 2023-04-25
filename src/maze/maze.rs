use std::usize;

const INNER_CIRCLE_PARTS: u32 = 5;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub step: u32,
}

#[derive(PartialEq)]
pub enum BorderType {
    Line,
    Arc,
}

#[derive(Debug)]
pub struct Border {
    pub start: CircleCoordinate,
    pub end: CircleCoordinate,
}

impl Border {
    pub fn border_type(&self) -> BorderType {
        if self.start.circle == self.end.circle {
            BorderType::Arc
        } else {
            BorderType::Line
        }
    }
}

pub fn steps_in_circle(circle: u32) -> u32 {
    (circle + 1) * INNER_CIRCLE_PARTS
}

pub fn create_maze(circles: u32) -> Vec<Border> {
    let mut maze_factory = MazeFactory {
        maze: Maze {
            outer_circle: circles,
            borders: Vec::new(),
        },
        open_coords: all_coords(circles),
    };

    maze_factory.create();
    maze_factory.maze.borders
}

enum StartEnd {
    Start,
    End,
}

struct Maze {
    outer_circle: u32,
    borders: Vec<Border>,
}

impl Maze {
    fn create_path(
        &mut self,
        start_coord: &CircleCoordinate,
        open_coords: &[CircleCoordinate],
    ) -> Vec<CircleCoordinate> {
        let mut current_path: Vec<CircleCoordinate> = vec![start_coord.to_owned()];
        let mut options: Vec<(CircleCoordinate, Direction)> = Vec::new();
        let mut coord = start_coord.to_owned();
        while open_coords.contains(&coord) {
            add_options(&mut options, &coord);
            let ((from_coord, direction), to_coord) = self.next(&mut options, &current_path);
            coord = to_coord.to_owned();
            current_path.push(to_coord.to_owned());
            match direction {
                Direction::In => {
                    self.merge_borders(from_coord, to_coord, BorderType::Line, StartEnd::End)
                }
                Direction::Out => {
                    self.merge_borders(from_coord, to_coord, BorderType::Line, StartEnd::Start)
                }
                Direction::Clockwise => {
                    self.merge_borders(from_coord, to_coord, BorderType::Arc, StartEnd::End)
                }
                Direction::CounterClockwise => {
                    self.merge_borders(from_coord, to_coord, BorderType::Arc, StartEnd::Start)
                }
            };
        }

        current_path
    }

    fn next(
        &self,
        options: &mut Vec<(CircleCoordinate, Direction)>,
        current_path: &[CircleCoordinate],
    ) -> ((CircleCoordinate, Direction), CircleCoordinate) {
        while !options.is_empty() {
            let option = options.remove(random_index(options.len()));
            let neighbour = self.neighbour(&option.0, &option.1);
            if let Some(coord) = neighbour {
                if !current_path.contains(&coord) {
                    return (option, coord);
                }
            }
        }

        panic!();
    }

    fn merge_borders(
        &mut self,
        from_coord: CircleCoordinate,
        to_coord: CircleCoordinate,
        border_type: BorderType,
        start_end: StartEnd,
    ) {
        let pos_option = match start_end {
            StartEnd::Start => self
                .borders
                .iter()
                .position(|b| b.border_type() == border_type && b.start == to_coord),
            StartEnd::End => self
                .borders
                .iter()
                .position(|b| b.border_type() == border_type && b.end == to_coord),
        };

        if let Some(pos) = pos_option {
            let old = self.borders.remove(pos);
            match start_end {
                StartEnd::Start => {
                    self.borders.push(Border {
                        start: from_coord,
                        end: old.end,
                    });
                }
                StartEnd::End => self.borders.push(Border {
                    start: old.start,
                    end: from_coord,
                }),
            };
        } else {
            self.borders.push(Border {
                start: from_coord,
                end: to_coord,
            });
        }
    }

    fn neighbour(
        &self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<CircleCoordinate> {
        match direction {
            Direction::In => {
                if coord.circle == 0 {
                    None
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle - 1,
                        step: coord.step,
                    })
                }
            }

            Direction::Out => {
                if coord.circle == (self.outer_circle - 1) {
                    None
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle + 1,
                        step: coord.step,
                    })
                }
            }

            Direction::Clockwise => {
                if coord.step == (steps_in_circle(coord.circle) - 1) {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        step: 0,
                    })
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        step: coord.step + 1,
                    })
                }
            }
            Direction::CounterClockwise => {
                if coord.step == 0 {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        step: steps_in_circle(coord.circle) - 1,
                    })
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        step: coord.step - 1,
                    })
                }
            }
        }
    }
}

struct MazeFactory {
    maze: Maze,
    open_coords: Vec<CircleCoordinate>,
}

impl MazeFactory {
    fn create(&mut self) {
        // while !self.open_coords.is_empty() {
            let coord = &self.open_coords[random_index(self.open_coords.len())];
            let path_coords = self.maze.create_path(coord, &self.open_coords);
            self.open_coords.retain(|e| !path_coords.contains(e));
            println!("{:?}", self.maze.borders);
        // }
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
    In,
    Out,
    Clockwise,
    CounterClockwise,
}

fn all_coords(circles: u32) -> Vec<CircleCoordinate> {
    let mut result: Vec<CircleCoordinate> = Vec::new();

    for circle in 0..(circles - 1) {
        for step in 0..steps_in_circle(circle) {
            result.push(CircleCoordinate { circle, step });
        }
    }

    result
}
