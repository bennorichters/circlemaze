use crate::maze::maze::{steps_in_circle, Border, BorderType};

const FULL_CIRCLE: f64 = 2. * std::f64::consts::PI;
const RADIUS_INNER_CIRCLE: u32 = 20;
const CENTER_X: u64 = 50;
const CENTER_Y: u64 = 50;

pub type CartesianCoord = (f64, f64);

pub trait Canvas {
    fn move_to(self, coord: CartesianCoord) -> Self;
    fn draw_arc(self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self;
    fn draw_line(self, coord: CartesianCoord) -> Self;
}

pub fn parse<T: Canvas>(borders: Vec<Border>, mut canvas: T) -> T {
    for border in borders {
        let total_steps = steps_in_circle(border.start.circle);
        let radius = (border.start.circle + 1) * RADIUS_INNER_CIRCLE;
        let angle = angle(border.start.step, total_steps);
        let coord = cartesian_coord(radius, angle);

        canvas = canvas.move_to(coord);
        canvas = match border.border_type {
            BorderType::Arc => {
                let (long_arc_flag, coord) =
                    arc(radius, border.start.step, total_steps, border.length);
                canvas.draw_arc(radius, long_arc_flag, coord)
            }

            BorderType::Line => canvas.draw_line(line(border.start.circle, angle, border.length)),
        };
    }

    canvas
}

fn arc(radius: u32, start_step: u32, total_steps: u32, length: u32) -> (u8, CartesianCoord) {
    let end_angle = angle(start_step + length, total_steps);
    let large_arc_flag: u8 = (length > (total_steps / 2)).into();
    let coord = cartesian_coord(radius, end_angle);

    (large_arc_flag, coord)
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

#[cfg(test)]
mod parse_tests {
    extern crate approx;

    use approx::abs_diff_eq;

    use crate::{
        maze::maze::{Border, BorderType, CircleCoordinate},
        svg::parse::{parse, Canvas},
    };

    use super::CartesianCoord;

    #[test]
    fn test_parse() {
        let path = vec![
            Border {
                border_type: BorderType::Arc,
                start: CircleCoordinate { circle: 0, step: 0 },
                length: 3,
            },
            Border {
                border_type: BorderType::Line,
                start: CircleCoordinate { circle: 0, step: 2 },
                length: 1,
            },
        ];
        let expected = DataHolder {
            params: vec![
                Param::Move((70., 50.)),
                Param::Arc(20, 1, (33.81966, 61.755707)),
                Param::Move((33.81966, 38.244293)),
                Param::Line((17.63932, 26.48859)),
            ],
            index: 0,
        };
        parse(path, expected);
    }

    const EPSILON: f64 = 0.00001;

    enum Param {
        Move(CartesianCoord),
        Arc(u32, u8, CartesianCoord),
        Line(CartesianCoord),
    }

    struct DataHolder {
        params: Vec<Param>,
        index: usize,
    }

    impl DataHolder {
        fn end(mut self) -> Self {
            self.index += 1;
            self
        }
    }

    impl Canvas for DataHolder {
        fn move_to(self, coord: CartesianCoord) -> Self {
            if let Param::Move(exp_coord) = self.params[self.index] {
                assert!(abs_diff_eq!(coord.0, exp_coord.0, epsilon = EPSILON));
                assert!(abs_diff_eq!(coord.1, exp_coord.1, epsilon = EPSILON));

                return self.end();
            }
            panic!();
        }

        fn draw_arc(self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self {
            if let Param::Arc(exp_radius, exp_flag, exp_coord) = self.params[self.index] {
                assert_eq!(radius, exp_radius);
                assert_eq!(long_arc_flag, exp_flag);
                assert!(abs_diff_eq!(coord.0, exp_coord.0, epsilon = EPSILON));
                assert!(abs_diff_eq!(coord.1, exp_coord.1, epsilon = EPSILON));

                return self.end();
            }
            panic!();
        }

        fn draw_line(self, coord: CartesianCoord) -> Self {
            if let Param::Line(exp_coord) = self.params[self.index] {
                assert!(abs_diff_eq!(coord.0, exp_coord.0, epsilon = EPSILON));
                assert!(abs_diff_eq!(coord.1, exp_coord.1, epsilon = EPSILON));

                return self.end();
            }
            panic!()
        }
    }
}
