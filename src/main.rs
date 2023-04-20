use maze::maze::{Border, CircleCoordinate, Direction};

mod maze;
mod svg;

fn main() {
    let abc = Border {
        start: CircleCoordinate { circle: 0, step: 0 },
        direction: Direction::Clockwise,
        length: 3,
    };

    let def = Border {
        start: CircleCoordinate { circle: 0, step: 2 },
        direction: Direction::Out,
        length: 1,
    };

    svg::draw::draw(vec![abc, def]);
}
