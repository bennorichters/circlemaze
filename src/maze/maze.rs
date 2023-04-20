const INNER_CIRCLE_PARTS: u32 = 5;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CircleCoordinate {
    pub circle: u32,
    pub step: u32,
}

pub enum Direction {
    In,
    Out,
    Clockwise,
    CounterClockwise,
}

pub struct Border {
    pub start: CircleCoordinate,
    pub direction: Direction,
    pub length: u32,
}

fn all_coords(circles: u32) -> Vec<CircleCoordinate> {
    let mut result: Vec<CircleCoordinate> = Vec::new();

    for circle in 0..circles {
        for step in 0..steps_in_circle(circle) {
            result.push(CircleCoordinate { circle, step });
        }
    }

    result
}

pub fn steps_in_circle(circle: u32) -> u32 {
    (circle + 1) * INNER_CIRCLE_PARTS
}
