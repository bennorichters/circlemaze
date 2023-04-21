use crate::maze::maze::{steps_in_circle, Border, BorderType};

const FULL_CIRCLE: f64 = 2. * std::f64::consts::PI;
const RADIUS_INNER_CIRCLE: u32 = 20;
const CENTER_X: u64 = 50;
const CENTER_Y: u64 = 50;

pub trait Canvas {
    fn move_to(self, coord: (f64, f64)) -> Self;
    fn draw_arc(self, radius: u32, params: (u8, f64, f64)) -> Self;
    fn draw_line(self, params: (f64, f64)) -> Self;
}

pub fn parse<T: Canvas>(borders: Vec<Border>, mut canvas: T) -> T {
    for border in borders {
        let total_steps = steps_in_circle(border.start.circle);
        let radius = (border.start.circle + 1) * RADIUS_INNER_CIRCLE;
        let angle = angle(border.start.step, total_steps);
        let coord = cartesian_coord(radius, angle);

        canvas = canvas.move_to(coord);
        canvas = match border.border_type {
            BorderType::Arc => canvas.draw_arc(
                radius,
                arc(radius, border.start.step, total_steps, border.length),
            ),

            BorderType::Line => canvas.draw_line(line(border.start.circle, angle, border.length)),
        };
    }

    canvas
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
