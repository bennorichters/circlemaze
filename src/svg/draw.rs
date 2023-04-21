use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::maze::maze::Border;

use super::parse::{parse, Canvas};

pub fn draw(borders: Vec<Border>) {
    let data = Data::new();
    let canvas = SvgCanvas { data };
    let canvas = parse(borders, canvas);

    let path = Path::new()
        .set("d", canvas.data)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);

    svg::save("image.svg", &document).unwrap();
}

struct SvgCanvas {
    data: Data,
}

impl Canvas for SvgCanvas {
    fn data_move(mut self, coord: (f64, f64)) -> Self {
        self.data = self.data.move_to(coord);
        self
    }

    fn data_arc(mut self, radius: u32, params: (u8, f64, f64)) -> Self {
        self.data = self.data
            .elliptical_arc_to((radius, radius, 0, params.0, 0, params.1, params.2));
        self
    }

    fn data_line(mut self, params: (f64, f64)) -> Self {
        self.data = self.data.line_to(params);
        self
    }
}
