use fraction::GenericFraction;

#[derive(Debug)]
pub enum Direction {
    Out,
    In,
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, PartialEq)]
pub enum CellState {
    Free,
    Taken,
}

pub trait Grid {
    fn dist(&mut self) -> Box<dyn Distributor>;
}

pub trait Distributor {
    fn take_from_outer_circle(&mut self) -> (CircleCoordinate, CellState);
    fn consume_outer_circle(&mut self);
    fn take_free(&mut self) -> Option<CircleCoordinate>;
    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<(CircleCoordinate, CellState)>;
}

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

pub fn random_nr(upper_bound: usize) -> usize {
    (rand::random::<f32>() * upper_bound as f32).floor() as usize
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub angle: Angle,
}

#[cfg(test)]
mod components_test {
    use crate::maze::{
        components::BorderType,
        test_utils::helper_fns::{create_border, create_coord},
    };

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
    fn test_ordering_circular_coordinate() {
        assert!(create_coord(0, 0, 1) < create_coord(0, 1, 2));
        assert!(create_coord(0, 1, 2) < create_coord(1, 0, 1));
        assert!(create_coord(0, 1, 2) < create_coord(0, 3, 4));
        assert!(create_coord(0, 3, 20) < create_coord(0, 1, 4));
        assert!(create_coord(2, 1, 4) < create_coord(3, 3, 20));
    }
}
