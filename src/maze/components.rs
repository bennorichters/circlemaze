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
        let numer = *self.angle.numer().unwrap();
        let denom = self.angle.denom().unwrap();

        self.angle.is_zero()
            || ((denom % slices == 0)
                && ((denom % (self.circle + 1) == 0)
                    || ((denom % self.circle == 0)
                        && Angle::new(numer, slices).to_f64().unwrap() >= min_dist
                        && Angle::new(slices - numer, slices).to_f64().unwrap() >= min_dist)))
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
        on_grid_fail(1, 1, 3, 3, 1.);
        on_grid_fail(1, 1, 3, 3, 0.334);
        on_grid_fail(1, 2, 3, 3, 0.334);

        on_grid_fail(2, 9, 14, 7, 0.2);
    }
}
