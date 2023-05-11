#[cfg(test)]
pub mod helper_fns {
    use crate::maze::components::{Angle, Border, CircleCoordinate};

    pub fn create_coord(circle: u32, numer: u32, denom: u32) -> CircleCoordinate {
        CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom),
        }
    }

    pub fn create_border(
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
}
