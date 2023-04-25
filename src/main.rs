use maze::maze::create_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let borders = create_maze(3);
    _ = draw::draw(borders);
}
