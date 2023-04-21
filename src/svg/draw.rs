use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::maze::maze::Border;

use super::parse::{parse, Canvas, CartesianCoord};

pub fn draw(borders: Vec<Border>) {
    let canvas = parse(borders, SvgCanvas { data: Data::new() });

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
    fn move_to(mut self, coord: CartesianCoord) -> Self {
        self.data = self.data.move_to(coord);
        self
    }

    fn draw_arc(mut self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self {
        self.data = self
            .data
            .elliptical_arc_to((radius, radius, 0, long_arc_flag, coord));
        self
    }

    fn draw_line(mut self, coord: CartesianCoord) -> Self {
        self.data = self.data.line_to(coord);
        self
    }
}
