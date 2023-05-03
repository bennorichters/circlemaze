use super::{
    circular_grid::CircularGrid,
    components::{Angle, Border, BorderType, CircleCoordinate},
};

#[derive(Debug)]
pub enum Direction {
    Out,
    In,
    Clockwise,
    CounterClockwise,
}

pub trait Grid {
    fn next(
        &self,
        options: &mut Vec<(CircleCoordinate, Direction)>,
        current_path: &[CircleCoordinate],
    ) -> (CircleCoordinate, CircleCoordinate, Direction);
}

pub fn create_maze(circles: u32, inner_slices: u32, min_dist: f64) -> Vec<Border> {
    let grid = CircularGrid {
        outer_circle: circles,
        inner_slices,
        min_dist,
    };

    let borders: Vec<Border> = vec![Border {
        start: CircleCoordinate {
            circle: circles,
            angle: Angle::from(0),
        },
        end: CircleCoordinate {
            circle: circles,
            angle: Angle::from(0),
        },
    }];

    let mut open_coords = grid.all_coords();
    let mut maze = Maze { borders, grid };

    while !open_coords.is_empty() {
        let coord = &open_coords[random_index(open_coords.len())];
        let path_coords = maze.create_path(coord, &open_coords);
        open_coords.retain(|e| !path_coords.contains(e));
    }
    maze.borders
}

struct Maze {
    grid: CircularGrid,
    borders: Vec<Border>,
}

impl Maze {
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
            let (from_coord, to_coord, direction) = self.grid.next(&mut options, &visited);
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
