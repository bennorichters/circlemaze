use crate::maze::{circular_grid::build_grid, components::Grid, maze_builder::build_maze};

mod maze;

fn main() {
    let circles = 5;
    let base_segments = 10;
    let min_distance = 0.3;

    let grid = build_grid(circles - 1, base_segments, min_distance);
    let dist = grid.dist();
    let borders = build_maze(&dist);

    println!("{:?}", borders);
}
