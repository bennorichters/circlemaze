use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::maze::maze::Border;

use super::parse::{parse, Canvas};

pub fn draw(borders: Vec<Border>) {
    let mut data = Data::new();
    data = parse(borders, data, Aap {});

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);

    svg::save("image.svg", &document).unwrap();
}

struct Aap;

impl Canvas<Data> for Aap {
    fn data_move(&mut self, data: Data, coord: (f64, f64)) -> Data {
        data.move_to(coord)
    }

    fn data_arc(&mut self, data: Data, radius: u32, params: (u8, f64, f64)) -> Data {
        data.elliptical_arc_to((radius, radius, 0, params.0, 0, params.1, params.2))
    }

    fn data_line(&mut self, data: Data, params: (f64, f64)) -> Data {
        data.line_to(params)
    }
}
