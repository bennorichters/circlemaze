use maze::factory::create_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let borders = create_maze(15, 9, 0.2);
    _ = draw::draw(borders);
}
