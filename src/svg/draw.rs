use handlebars::Handlebars;
use std::{collections::HashMap, error::Error, fs::File};

use crate::maze::components::Border;

use super::parse::{Canvas, CartesianCoord, Parser};

pub fn draw(borders: Vec<Border>) -> Result<(), Box<dyn Error>> {
    let parser = Parser{ center: (400., 350.), radius_inner_circle: 20, borders };
    let canvas = parser.parse(
        SvgCanvas {
            path: String::new(),
            circle: None,
        },
    );

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("path".to_string(), canvas.path);
    if let Some(circle) = canvas.circle {
        data.insert("circle_center_x".to_string(), circle.center_x);
        data.insert("circle_center_y".to_string(), circle.center_y);
        data.insert("circle_radius".to_string(), circle.radius);
    }

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("t1", "./assets/maze.template.svg")
        .unwrap();

    let mut output_file = File::create("maze.svg")?;
    handlebars.render_to_write("t1", &data, &mut output_file)?;
    Ok(())
}

struct Circle {
    center_x: String,
    center_y: String,
    radius: String,
}

struct SvgCanvas {
    path: String,
    circle: Option<Circle>,
}

impl Canvas for SvgCanvas {
    fn move_to(mut self, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!("M {} {} ", coord.0, coord.1));
        self
    }

    fn draw_arc(mut self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!(
            "A {} {} 0 {} 0 {} {} ",
            radius, radius, long_arc_flag, coord.0, coord.1
        ));
        self
    }

    fn draw_circle(mut self, radius: u32, center: CartesianCoord) -> Self {
        self.circle = Some(Circle {
            center_x: center.0.to_string(),
            center_y: center.1.to_string(),
            radius: radius.to_string(),
        });
        self
    }

    fn draw_line(mut self, coord: CartesianCoord) -> Self {
        self.path.push_str(&format!("L {} {} ", coord.0, coord.1));
        self
    }
}
