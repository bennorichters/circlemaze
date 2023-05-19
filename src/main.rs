use maze::maze_builder::build_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let circles = 5;
    let base_segments = 10;
    let min_distance = 0.3;
    let borders = build_maze(circles, base_segments, min_distance);
    _ = draw::draw(circles as usize, borders);
}
