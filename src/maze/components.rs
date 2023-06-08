use fraction::GenericFraction;

pub trait Grid {
    type Dist<'a>: Distributor + 'a
    where
        Self: 'a;
    fn dist(&self) -> Self::Dist<'_>;
}

pub trait Distributor {
    fn take_from_outer_circle(&mut self) -> (&CircleCoordinate, CellState);
    fn consume_outer_circle(&mut self);
    fn take_free(&mut self) -> Option<&CircleCoordinate>;
    fn take_neighbour(
        &mut self,
        coord: &CircleCoordinate,
        direction: &Direction,
    ) -> Option<(&CircleCoordinate, CellState)>;
}

pub type Angle = GenericFraction<u32>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub angle: Angle,
}

impl<'a> PartialEq<&'a CircleCoordinate> for CircleCoordinate {
    fn eq(&self, other: &&'a CircleCoordinate) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<CircleCoordinate> for &'a CircleCoordinate {
    fn eq(&self, other: &CircleCoordinate) -> bool {
        *self == other
    }
}

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

#[derive(Debug, PartialEq)]
pub enum BorderType {
    Arc,
    Line,
}

pub fn random_nr(upper_bound: usize) -> usize {
    (rand::random::<f32>() * upper_bound as f32).floor() as usize
}
