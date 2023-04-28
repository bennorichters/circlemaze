const INNER_CIRCLE_PARTS: u32 = 5;

pub type Angle = (u32, u32);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub angle: Angle,
}

#[derive(PartialEq)]
pub enum BorderType {
    Arc,
    Line,
}

#[derive(Debug)]
pub enum Border {
    Arc(CircleCoordinate, CircleCoordinate),
    LineGrid(CircleCoordinate, CircleCoordinate),
    LineIn(CircleCoordinate),
    LineOut(CircleCoordinate),
}

impl Border {
    pub fn border_type(&self) -> BorderType {
        match *self {
            Border::Arc(_, _) => BorderType::Arc,
            _ => BorderType::Line,
        }
    }

    pub fn start(&self) -> &CircleCoordinate {
        match self {
            Border::Arc(result, _) => result,
            Border::LineGrid(result, _) => result,
            Border::LineIn(result) => result,
            Border::LineOut(result) => result,
        }
    }

    pub fn end(&self) -> &CircleCoordinate {
        match self {
            Border::Arc(_, result) => result,
            Border::LineGrid(_, result) => result,
            Border::LineIn(_) => panic!(),
            Border::LineOut(_) => panic!(),
        }
    }
}

pub fn steps_in_circle(circle: u32) -> u32 {
    (circle + 1) * INNER_CIRCLE_PARTS
}

fn all_coords(circles: u32) -> Vec<CircleCoordinate> {
    let mut result: Vec<CircleCoordinate> = Vec::new();

    for circle in 0..circles {
        let denominator = steps_in_circle(circle);
        for step in 0..denominator {
            result.push(CircleCoordinate {
                circle,
                angle: (step, denominator),
            });
        }
    }

    result
}

pub fn create_maze(circles: u32) -> Vec<Border> {
    let mut maze = Maze {
        outer_circle: circles,
        borders: Vec::new(),
    };
    let mut open_coords = all_coords(circles);

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
    borders: Vec<Border>,
}

impl Maze {
    fn close_outer_circle(&mut self) {
        let denominator = steps_in_circle(self.outer_circle);
        self.borders.push(Border::Arc(
            CircleCoordinate {
                circle: self.outer_circle,
                angle: (0, denominator),
            },
            CircleCoordinate {
                circle: self.outer_circle,
                angle: (0, denominator),
            },
        ));
    }

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
            let (merge_start, merge_end, border_type) = match direction {
                Direction::Out => (from_coord, to_coord, BorderType::Line),
                Direction::In => (to_coord, from_coord, BorderType::Line),
                Direction::Clockwise => (from_coord, to_coord, BorderType::Arc),
                Direction::CounterClockwise => (to_coord, from_coord, BorderType::Arc),
            };
            self.merge_borders(merge_start, merge_end, border_type);
        }

        current_path
    }

    // fn is_free(used: &[CircleCoordinate]) {
    // }

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
        start: CircleCoordinate,
        end: CircleCoordinate,
        border_type: BorderType,
    ) {
        let mut merged_start = start;
        let mut merged_end = end;

        if let Some(before_index) = self.find_merge_start(&merged_start, &border_type) {
            let before = self.borders.remove(before_index);
            merged_start = before.start().to_owned();
        }

        if let Some(after_index) = self.find_merge_end(&merged_end, &border_type) {
            let after = self.borders.remove(after_index);
            merged_end = after.end().to_owned();
        }

        self.borders.push(match border_type {
            BorderType::Arc => Border::Arc(merged_start, merged_end),
            BorderType::Line => Border::LineGrid(merged_start, merged_end),
        });
    }

    fn find_merge_start(
        &self,
        from_coord: &CircleCoordinate,
        border_type: &BorderType,
    ) -> Option<usize> {
        self.borders
            .iter()
            .position(|b| &b.border_type() == border_type && b.end() == from_coord)
    }

    fn find_merge_end(
        &self,
        to_coord: &CircleCoordinate,
        border_type: &BorderType,
    ) -> Option<usize> {
        self.borders
            .iter()
            .position(|b| &b.border_type() == border_type && b.start() == to_coord)
    }

    fn neighbour(
        &self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<CircleCoordinate> {
        match direction {
            Direction::In => {
                if coord.circle == 0 || (coord.circle * coord.angle.0) % (coord.circle + 1) != 0 {
                    None
                } else {
                    let numerator = (coord.circle * coord.angle.0) / (coord.circle + 1);
                    let denominator = steps_in_circle(coord.circle - 1);
                    Some(CircleCoordinate {
                        circle: coord.circle - 1,
                        angle: (numerator, denominator),
                    })
                }
            }

            Direction::Out => {
                if coord.circle == self.outer_circle
                    || ((coord.circle + 2) * coord.angle.0) % (coord.circle + 1) != 0
                {
                    None
                } else {
                    let numerator =((coord.circle + 2) * coord.angle.0) / (coord.circle + 1);
                    let denominator = steps_in_circle(coord.circle + 1);
                    Some(CircleCoordinate {
                        circle: coord.circle + 1,
                        angle: (numerator, denominator),
                    })
                }
            }

            Direction::Clockwise => {
                if coord.angle.0 == (steps_in_circle(coord.circle) - 1) {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        angle: (0, coord.angle.1),
                    })
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        angle: (coord.angle.0 + 1, coord.angle.1),
                    })
                }
            }
            Direction::CounterClockwise => {
                if coord.angle.0 == 0 {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        angle: (steps_in_circle(coord.circle) - 1, coord.angle.1),
                    })
                } else {
                    Some(CircleCoordinate {
                        circle: coord.circle,
                        angle: (coord.angle.0 - 1, coord.angle.1),
                    })
                }
            }
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
