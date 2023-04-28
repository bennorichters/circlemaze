use fraction::ToPrimitive;

use crate::maze::components::{Angle, Border, BorderType, CircleCoordinate};

const FULL_CIRCLE: f64 = 2. * std::f64::consts::PI;

pub type CartesianCoord = (f64, f64);

pub trait Canvas {
    fn move_to(self, coord: CartesianCoord) -> Self;
    fn draw_arc(self, radius: u32, long_arc_flag: u8, coord: CartesianCoord) -> Self;
    fn draw_circle(self, radius: u32, center: CartesianCoord) -> Self;
    fn draw_line(self, coord: CartesianCoord) -> Self;
}

pub struct Parser {
    pub center: CartesianCoord,
    pub radius_inner_circle: u32,
    pub borders: Vec<Border>,
}

impl Parser {
    pub fn parse<T: Canvas>(&self, mut canvas: T) -> T {
        for border in &self.borders {
            let radius = (border.start.circle + 1) * self.radius_inner_circle;

            canvas = if border.border_type() == BorderType::Arc && border.start == border.end {
                canvas.draw_circle(radius, self.center)
            } else {
                self.arc_or_line(border, radius, canvas)
            };
        }

        canvas
    }

    fn arc_or_line<T: Canvas>(&self, border: &Border, radius: u32, mut canvas: T) -> T {
        let angle = self.angle(border.start.angle);
        let coord = self.cartesian_coord(radius, angle);

        canvas = canvas.move_to(coord);
        match border.border_type() {
            BorderType::Arc => {
                let (long_arc_flag, coord) = self.arc(radius, border.start.angle, &border.end);
                canvas.draw_arc(radius, long_arc_flag, coord)
            }

            BorderType::Line => canvas.draw_line(self.line(angle, &border.end)),
        }
    }

    fn arc(&self, radius: u32, start_angle: Angle, end: &CircleCoordinate) -> (u8, CartesianCoord) {
        let diff = if start_angle > end.angle {
            Angle::from(1) - start_angle + end.angle
        } else {
            end.angle - start_angle
        };
        let large_arc_flag = (diff > Angle::new(1_u32, 2_u32)).into();

        let end_coord = self.cartesian_coord(radius, self.angle(end.angle));

        (large_arc_flag, end_coord)
    }

    fn line(&self, angle: f64, end: &CircleCoordinate) -> (f64, f64) {
        let end_radius = (end.circle + 1) * self.radius_inner_circle;
        self.cartesian_coord(end_radius, angle)
    }

    fn angle(&self, angle: Angle) -> f64 {
        FULL_CIRCLE * angle.to_f64().unwrap()
    }

    fn cartesian_coord(&self, radius: u32, angle: f64) -> (f64, f64) {
        (
            self.center.0 + radius as f64 * angle.cos(),
            self.center.1 - radius as f64 * angle.sin(),
        )
    }
}

#[cfg(test)]
mod parse_tests {
    extern crate approx;

    use approx::abs_diff_eq;

    use crate::{
        maze::components::{Angle, Border, CircleCoordinate},
        svg::parse::Canvas,
    };

    use super::{CartesianCoord, Parser};

    #[test]
    fn test_parse() {
        let path = vec![
            Border {
                start: CircleCoordinate {
                    circle: 0,
                    angle: Angle::from(0),
                },
                end: CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(3_u32, 5_u32),
                },
            },
            Border {
                start: CircleCoordinate {
                    circle: 0,
                    angle: Angle::new(2_u32, 5_u32),
                },
                end: CircleCoordinate {
                    circle: 1,
                    angle: Angle::new(4_u32, 10_u32),
                },
            },
            Border {
                start: CircleCoordinate {
                    circle: 2,
                    angle: Angle::from(0),
                },
                end: CircleCoordinate {
                    circle: 2,
                    angle: Angle::from(0),
                },
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

        let parser = Parser {
            center: (50., 50.),
            radius_inner_circle: 20,
            borders: path,
        };
        parser.parse(expected);
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
