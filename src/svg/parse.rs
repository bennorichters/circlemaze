use crate::maze::maze::{steps_in_circle, Border, BorderType, CircleCoordinate};

const FULL_CIRCLE: f64 = 2. * std::f64::consts::PI;
const RADIUS_INNER_CIRCLE: u32 = 20;
const CENTER: CartesianCoord = (400., 350.);

pub type CartesianCoord = (f64, f64);

pub trait Canvas {
    fn move_to(self, coord: CartesianCoord) -> Self;
    fn draw_arc(self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self;
    fn draw_circle(self, radius: u32, center: CartesianCoord) -> Self;
    fn draw_line(self, coord: CartesianCoord) -> Self;
}

pub fn parse<T: Canvas>(borders: Vec<Border>, mut canvas: T) -> T {
    for border in borders {
        let radius = (border.start.circle + 1) * RADIUS_INNER_CIRCLE;

        canvas = if border.border_type() == BorderType::Arc && border.start == border.end {
            canvas.draw_circle(radius, CENTER)
        } else {
            arc_or_line(border, radius, canvas)
        };
    }

    canvas
}

fn arc_or_line<T: Canvas>(border: Border, radius: u32, mut canvas: T) -> T {
    let total_steps = steps_in_circle(border.start.circle);
    let angle = angle(border.start.step, total_steps);
    let coord = cartesian_coord(radius, angle);

    canvas = canvas.move_to(coord);
    match border.border_type() {
        BorderType::Arc => {
            let (long_arc_flag, coord) = arc(radius, border.start.step, total_steps, border.end);
            canvas.draw_arc(radius, long_arc_flag, coord)
        }

        BorderType::Line => canvas.draw_line(line(angle, border.end)),
    }
}

fn arc(
    radius: u32,
    start_step: u32,
    total_steps: u32,
    end: CircleCoordinate,
) -> (u8, CartesianCoord) {
    let end_angle = angle(end.step, total_steps);
    let diff = if end.step >= start_step {
        end.step - start_step
    } else {
        total_steps - start_step + end.step
    };
    let large_arc_flag: u8 = (diff > (total_steps / 2)).into();
    let end_coord = cartesian_coord(radius, end_angle);

    (large_arc_flag, end_coord)
}

fn line(angle: f64, end: CircleCoordinate) -> (f64, f64) {
    let end_radius = (end.circle + 1) * RADIUS_INNER_CIRCLE;
    cartesian_coord(end_radius, angle)
}

fn angle(step: u32, total_steps: u32) -> f64 {
    FULL_CIRCLE * step as f64 / total_steps as f64
}

fn cartesian_coord(radius: u32, angle: f64) -> (f64, f64) {
    (
        CENTER.0 + radius as f64 * angle.cos(),
        CENTER.1 - radius as f64 * angle.sin(),
    )
}

#[cfg(test)]
mod parse_tests {
    extern crate approx;

    use approx::abs_diff_eq;

    use crate::{
        maze::maze::{Border, CircleCoordinate},
        svg::parse::{parse, Canvas},
    };

    use super::CartesianCoord;

    #[test]
    fn test_parse() {
        let path = vec![
            Border {
                start: CircleCoordinate { circle: 0, step: 0 },
                end: CircleCoordinate { circle: 0, step: 3 },
            },
            Border {
                start: CircleCoordinate { circle: 0, step: 2 },
                end: CircleCoordinate { circle: 1, step: 2 },
            },
            Border {
                start: CircleCoordinate { circle: 2, step: 0 },
                end: CircleCoordinate { circle: 2, step: 0 },
            },
        ];
        let expected = DataHolder {
            params: vec![
                Param::Move((70., 50.)),
                Param::Arc(20, 1, (33.81966, 61.755707)),
                Param::Move((33.81966, 38.244293)),
                Param::Line((17.63932, 26.48859)),
                Param::Circle(60, (50., 50.)),
            ],
            index: 0,
        };
        parse(path, expected);
    }

    const EPSILON: f64 = 0.00001;

    enum Param {
        Move(CartesianCoord),
        Arc(u32, u8, CartesianCoord),
        Circle(u32, CartesianCoord),
        Line(CartesianCoord),
    }

    struct DataHolder {
        params: Vec<Param>,
        index: usize,
    }

    impl DataHolder {
        fn test_coord(self, actual: CartesianCoord, expected: CartesianCoord) -> Self {
            assert!(abs_diff_eq!(actual.0, expected.0, epsilon = EPSILON));
            assert!(abs_diff_eq!(actual.1, expected.1, epsilon = EPSILON));
            self.end()
        }

        fn end(mut self) -> Self {
            self.index += 1;
            self
        }
    }

    impl Canvas for DataHolder {
        fn move_to(self, coord: CartesianCoord) -> Self {
            if let Param::Move(exp_coord) = self.params[self.index] {
                return self.test_coord(coord, exp_coord);
            }

            panic!();
        }

        fn draw_arc(self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self {
            if let Param::Arc(exp_radius, exp_flag, exp_coord) = self.params[self.index] {
                assert_eq!(radius, exp_radius);
                assert_eq!(long_arc_flag, exp_flag);
                return self.test_coord(coord, exp_coord);
            }

            panic!();
        }

        fn draw_circle(self, radius: u32, center: CartesianCoord) -> Self {
            if let Param::Circle(exp_radius, exp_center) = self.params[self.index] {
                assert_eq!(radius, exp_radius);
                assert_eq!(center, exp_center);
                return self.test_coord(center, exp_center);
            }

            panic!();
        }

        fn draw_line(self, coord: CartesianCoord) -> Self {
            if let Param::Line(exp_coord) = self.params[self.index] {
                return self.test_coord(coord, exp_coord);
            }

            panic!()
        }
    }
}
