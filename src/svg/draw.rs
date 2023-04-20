use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::maze::maze::{steps_in_circle, Border, BorderType};

const FULL_CIRCLE: f64 = 2. * std::f64::consts::PI;
const RADIUS_INNER_CIRCLE: u32 = 20;
const CENTER_X: u64 = 50;
const CENTER_Y: u64 = 50;

pub fn draw(borders: Vec<Border>) {
    let mut data = Data::new();

    for border in borders {
        let total_steps = steps_in_circle(border.start.circle);
        let radius = (border.start.circle + 1) * RADIUS_INNER_CIRCLE;
        let angle = angle(border.start.step, total_steps);
        let coord = cartesian_coord(radius, angle);

        data = data.move_to(coord);
        data = match border.border_type {
            BorderType::Arc => {
                let params = arc(radius, border.start.step, total_steps, border.length);
                data.elliptical_arc_to((radius, radius, 0, params.0, 0, params.1, params.2))
            }
            BorderType::Line => {
                let circle = border.start.circle;
                data.line_to(line(circle, angle, border.length))
            }
        };
    }

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(path);

    svg::save("image.svg", &document).unwrap();
}

fn arc(radius: u32, start_step: u32, total_steps: u32, length: u32) -> (u8, f64, f64) {
    let end_angle = angle(start_step + length, total_steps);
    let large_arc_flag: u8 = (length > (total_steps / 2)).into();
    let (end_x, end_y) = cartesian_coord(radius, end_angle);

    (large_arc_flag, end_x, end_y)
}

fn line(circle: u32, angle: f64, length: u32) -> (f64, f64) {
    let end_radius = (circle + length + 1) * RADIUS_INNER_CIRCLE;
    cartesian_coord(end_radius, angle)
}

fn angle(step: u32, total_steps: u32) -> f64 {
    FULL_CIRCLE * step as f64 / total_steps as f64
}

fn cartesian_coord(radius: u32, angle: f64) -> (f64, f64) {
    (
        CENTER_X as f64 + radius as f64 * angle.cos(),
        CENTER_Y as f64 - radius as f64 * angle.sin(),
    )
}
