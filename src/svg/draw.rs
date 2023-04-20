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
        let info = start_info(&border);
        data = data.move_to(info.coord);
        data = match border.border_type {
            BorderType::Arc => {
                let params = arc(&info);
                data.elliptical_arc_to((
                    info.radius,
                    info.radius,
                    0,
                    params.0,
                    0,
                    params.1,
                    params.2,
                ))
            }
            BorderType::Line => data.line_to(line(&info)),
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

fn arc(info: &BorderInfo) -> (u8, f64, f64) {
    let end_angle = angle(info.start_step + info.length, info.total_steps);
    let large_arc_flag: u8 = (info.length > (info.total_steps / 2)).into();
    let (end_x, end_y) = cartesian_coord(info.radius, end_angle);

    (large_arc_flag, end_x, end_y)
}

fn line(info: &BorderInfo) -> (f64, f64) {
    let end_radius = (info.circle + info.length + 1) * RADIUS_INNER_CIRCLE;
    cartesian_coord(end_radius, info.angle)
}

struct BorderInfo {
    circle: u32,
    total_steps: u32,
    start_step: u32,
    radius: u32,
    angle: f64,
    coord: (f64, f64),
    length: u32,
}

fn start_info(border: &Border) -> BorderInfo {
    let circle = border.start.circle;
    let total_steps = steps_in_circle(border.start.circle);
    let start_step = border.start.step;
    let radius = (border.start.circle + 1) * RADIUS_INNER_CIRCLE;
    let angle = angle(start_step, total_steps);
    let coord = cartesian_coord(radius, angle);

    BorderInfo {
        circle,
        total_steps,
        start_step,
        radius,
        angle,
        coord,
        length: border.length,
    }
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
