use maze::maze::create_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let borders = create_maze(15);
    _ = draw::draw(borders);
}
