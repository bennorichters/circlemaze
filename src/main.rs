use maze::{maze_builder::build_maze, circular_grid};
use svg::draw;

mod maze;
mod svg;

fn main() {
    let circles = 5;
    let base_segments = 10;
    let min_distance = 0.3;
    let mut grid = circular_grid::build(circles - 1, base_segments, min_distance);
    let borders = build_maze(grid.dist());
    _ = draw::draw(circles as usize, borders);
}
