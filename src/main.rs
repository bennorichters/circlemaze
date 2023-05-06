use maze::maze_builder::build_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let borders = build_maze(15, 9, 0.2);
    _ = draw::draw(borders);
}
