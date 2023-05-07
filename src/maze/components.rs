use fraction::GenericFraction;

pub type Angle = GenericFraction<u32>;

#[derive(Debug, PartialEq)]
pub enum BorderType {
    Arc,
    Line,
}

#[derive(Debug)]
pub struct Border {
    pub start: CircleCoordinate,
    pub end: CircleCoordinate,
}

impl Border {
    pub fn border_type(&self) -> BorderType {
        if self.start.circle == self.end.circle {
            BorderType::Arc
        } else {
            BorderType::Line
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub angle: Angle,
}

#[cfg(test)]
mod components_test {
    use crate::maze::components::BorderType;

    use super::{Angle, Border, CircleCoordinate};

    fn create_border(
        circle1: u32,
        numer1: u32,
        denom1: u32,
        circle2: u32,
        numer2: u32,
        denom2: u32,
    ) -> Border {
        Border {
            start: CircleCoordinate {
                circle: circle1,
                angle: Angle::new(numer1, denom1),
            },
            end: CircleCoordinate {
                circle: circle2,
                angle: Angle::new(numer2, denom2),
            },
        }
    }

    #[test]
    fn test_border_type() {
        assert_eq!(
            BorderType::Arc,
            create_border(0, 0, 1, 0, 0, 1).border_type()
        );
        assert_eq!(
            BorderType::Arc,
            create_border(0, 0, 1, 0, 1, 15).border_type()
        );
        assert_eq!(
            BorderType::Line,
            create_border(0, 0, 1, 5, 0, 1).border_type()
        );
    }
}
