use fraction::{GenericFraction, ToPrimitive, Zero};

pub type Angle = GenericFraction<u32>;

#[derive(PartialEq)]
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

impl CircleCoordinate {
    pub fn is_on_grid(&self, slices: u32, min_dist: f64) -> bool {
        let denom = self.angle.denom().unwrap();
        if ((self.circle + 1) * slices) % denom == 0 {
            return true;
        }

        if self.circle == 0 {
            return false;
        }

        if (self.circle * slices) % denom != 0 {
            return false;
        }

        let section = (self.angle * slices).floor().to_u32().unwrap();
        let angle_in_section = self.angle - Angle::new(section, slices * self.circle);

        let dist = angle_in_section.to_f64().unwrap();
        min_dist <= dist && dist <= (1. - min_dist)
    }
}

#[cfg(test)]
mod components_test {
    use crate::maze::components::{Angle, CircleCoordinate};

    fn on_grid_pass(circle: u32, numer: u32, denom: u32, slices: u32, min_dist: f64) {
        assert!(CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom)
        }
        .is_on_grid(slices, min_dist));
    }

    fn on_grid_fail(circle: u32, numer: u32, denom: u32, slices: u32, min_dist: f64) {
        assert!(!CircleCoordinate {
            circle,
            angle: Angle::new(numer, denom)
        }
        .is_on_grid(slices, min_dist));
    }

    #[test]
    fn test_circle_coordinate_is_on_grid() {
        on_grid_pass(0, 0, 1, 3, 1.);
        on_grid_pass(0, 0, 1, 3, 0.);
        on_grid_pass(0, 1, 3, 3, 1.);
        on_grid_pass(0, 1, 3, 3, 0.);
        on_grid_pass(0, 2, 3, 3, 1.);
        on_grid_pass(0, 2, 3, 3, 0.);
        on_grid_pass(1, 1, 6, 3, 1.);
        on_grid_pass(1, 1, 6, 3, 0.);
        on_grid_pass(1, 5, 6, 3, 0.);
        on_grid_pass(1, 1, 3, 3, 0.);

        on_grid_pass(1, 1, 3, 3, 0.33);
        on_grid_pass(1, 2, 3, 3, 0.33);

        on_grid_fail(0, 1, 4, 3, 1.);

        on_grid_pass(4, 1, 28, 7, 0.);
        on_grid_fail(4, 1, 28, 7, 0.3);
        on_grid_pass(5, 1, 35, 7, 0.);
        on_grid_fail(5, 1, 35, 7, 0.3);
        on_grid_pass(5, 1, 35, 7, 0.);
        on_grid_fail(5, 1, 35, 7, 0.3);
        on_grid_fail(5, 4, 35, 7, 0.3);
        on_grid_pass(5, 6, 35, 7, 0.);
        on_grid_fail(5, 6, 35, 7, 0.3);

        on_grid_pass(3, 11, 21, 7, 0.2);
    }
}
