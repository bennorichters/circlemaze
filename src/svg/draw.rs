use handlebars::Handlebars;
use std::{collections::HashMap, error::Error, fs::File};

use crate::maze::maze::Border;

use super::parse::{parse, Canvas, CartesianCoord};

pub fn draw(borders: Vec<Border>) -> Result<(), Box<dyn Error>> {
    let canvas = parse(borders, SvgCanvas { path: String::new() });

    let mut handlebars = Handlebars::new();

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("path".to_string(), canvas.path);
    data.insert("circle".to_string(), "".to_string());

    handlebars
        .register_template_file("t1", "./assets/maze.template.svg")
        .unwrap();

    let mut output_file = File::create("maze.svg")?;
    handlebars.render_to_write("t1", &data, &mut output_file)?;
    Ok(())
}

struct SvgCanvas {
    path: String,
}

impl Canvas for SvgCanvas {
    fn move_to(mut self, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!("M {} {}", coord.0, coord.1));
        self
    }

    fn draw_arc(mut self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!(
            "A {} {} 0 {} 0 {} {}",
            radius, radius, long_arc_flag, coord.0, coord.1
        ));
        self
    }

    fn draw_circle(mut self, radius: u32, center: CartesianCoord) -> Self {
        self = self.move_to((center.0 + radius as f64, center.1));
        self = self.draw_arc(radius, 0, (center.0 - radius as f64, center.1));
        self = self.draw_arc(radius, 0, (center.0 + radius as f64, center.1));
        self
    }

    fn draw_line(mut self, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!("L {} {}", coord.0, coord.1));
        self
    }
}
