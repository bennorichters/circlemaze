use maze::maze::{Border, BorderType, CircleCoordinate};
use svg::draw;

mod maze;
mod svg;

fn main() {
    let path = vec![
        Border {
            border_type: BorderType::Arc,
            start: CircleCoordinate { circle: 0, step: 0 },
            end: CircleCoordinate { circle: 0, step: 3 },
        },
        Border {
            border_type: BorderType::Line,
            start: CircleCoordinate { circle: 0, step: 2 },
            end: CircleCoordinate { circle: 1, step: 2 },
        },
        Border {
            border_type: BorderType::Arc,
            start: CircleCoordinate { circle: 1, step: 0 },
            end: CircleCoordinate { circle: 1, step: 0 },
        },
    ];

    _ = draw::draw(path);
}
