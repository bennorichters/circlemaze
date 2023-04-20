use maze::maze::{Border, BorderType, CircleCoordinate};

mod maze;
mod svg;

fn main() {
    let abc = Border {
        border_type: BorderType::Arc,
        start: CircleCoordinate { circle: 0, step: 0 },
        length: 3,
    };

    let def = Border {
        border_type: BorderType::Line,
        start: CircleCoordinate { circle: 0, step: 2 },
        length: 1,
    };

    svg::draw::draw(vec![abc, def]);
}
