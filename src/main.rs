use maze::factory::create_maze;
use svg::draw;

mod maze;
mod svg;

fn main() {
    let borders = create_maze(15, 7);
    _ = draw::draw(borders);
}
