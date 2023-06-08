use crate::maze::components::random_nr;

use super::components::{Border, Distributor, CircleCoordinate, Direction, CellState, BorderType};

pub fn build_maze(dist: &impl Distributor) -> Vec<Border> {
    todo!()
}

struct MazeBuilder {
    dist: Box<dyn Distributor>,
    borders: Vec<Border>,
}

impl MazeBuilder {
    fn create_borders(&mut self) {
        let (outer_coord, _status) = self.dist.take_from_outer_circle();
        self.dist.consume_outer_circle();
        self.borders.push(Border {
            start: outer_coord.to_owned(),
            end: outer_coord.to_owned(),
        });
        // while let Some(coord) = self.dist.take_free() {
        //     self.create_path(&coord);
        // }
    }

    fn create_path(&mut self, start_coord: &CircleCoordinate) -> Vec<CircleCoordinate> {
        let mut visited: Vec<CircleCoordinate> = vec![start_coord.to_owned()];
        let mut options: Vec<(CircleCoordinate, Direction)> = Vec::new();
        let mut coord = start_coord.to_owned();
        let mut path_open = true;
        while path_open {
            add_options(&mut options, &coord);
            let (from_coord, to_coord, direction, state) = self.next(&mut options, &visited);
            coord = to_coord.to_owned();
            path_open = state == CellState::Free;

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
        &mut self,
        options: &mut Vec<(CircleCoordinate, Direction)>,
        current_path: &[CircleCoordinate],
    ) -> (CircleCoordinate, CircleCoordinate, Direction, CellState) {
        while !options.is_empty() {
            let (candidate_start, candidate_direction) = options.remove(random_nr(options.len()));
            let neighbour_option = self
                .dist
                .take_neighbour(&candidate_start, &candidate_direction);
            if let Some((end, status)) = neighbour_option {
                if !current_path.contains(&end) {
                    return (candidate_start, end.to_owned(), candidate_direction, status);
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
