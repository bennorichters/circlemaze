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

    pub fn contains(&self, coord: CircleCoordinate) -> bool {
        todo!();
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

    fn create_coord(circle: u32, numer: u32, denom: u32) -> CircleCoordinate {
        CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom),
        }
    }

    fn create_border(
        circle1: u32,
        numer1: u32,
        denom1: u32,
        circle2: u32,
        numer2: u32,
        denom2: u32,
    ) -> Border {
        Border {
            start: create_coord(circle1, numer1, denom1),
            end: create_coord(circle2, numer2, denom2),
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

    #[test]
    #[ignore]
    fn test_border_contains() {
        assert!(create_border(0, 0, 1, 0, 1, 10).contains(create_coord(0, 1, 20)));
        assert!(!create_border(0, 0, 1, 0, 1, 20).contains(create_coord(0, 1, 10)));

        assert!(create_border(0, 0, 1, 0, 0, 1).contains(create_coord(0, 1, 20)));
        assert!(!create_border(0, 0, 1, 0, 0, 1).contains(create_coord(1, 1, 10)));

        assert!(!create_border(1, 0, 1, 5, 0, 1).contains(create_coord(0, 0, 1)));
        assert!(create_border(1, 0, 1, 5, 0, 1).contains(create_coord(1, 0, 1)));
        assert!(create_border(1, 0, 1, 5, 0, 1).contains(create_coord(2, 0, 1)));
        assert!(create_border(1, 0, 1, 5, 0, 1).contains(create_coord(3, 0, 1)));
        assert!(create_border(1, 0, 1, 5, 0, 1).contains(create_coord(4, 0, 1)));
        assert!(create_border(1, 0, 1, 5, 0, 1).contains(create_coord(5, 0, 1)));
        assert!(!create_border(1, 0, 1, 5, 0, 1).contains(create_coord(6, 0, 1)));

        assert!(!create_border(1, 2, 3, 3, 2, 3).contains(create_coord(0, 2, 3)));
        assert!(create_border(1, 2, 3, 3, 2, 3).contains(create_coord(1, 2, 3)));
        assert!(create_border(1, 2, 3, 3, 2, 3).contains(create_coord(2, 2, 3)));
        assert!(create_border(1, 2, 3, 3, 2, 3).contains(create_coord(3, 2, 3)));
        assert!(!create_border(1, 2, 3, 3, 2, 3).contains(create_coord(4, 2, 3)));

        assert!(!create_border(1, 2, 3, 3, 2, 3).contains(create_coord(1, 2, 5)));
        assert!(!create_border(1, 2, 3, 3, 2, 3).contains(create_coord(2, 1, 3)));
    }
}
