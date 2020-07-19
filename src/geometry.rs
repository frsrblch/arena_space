use crate::components::{Length, Area};

pub struct Sphere<D> {
    dim: D,
}

impl Sphere<Length> {
    pub fn with_radius(radius: Length) -> Self {
        Sphere {
            dim: radius,
        }
    }

    pub fn get_area(&self) -> Area {
        let r_squared = self.dim * self.dim;
        FOUR_PI * r_squared
    }
}

const FOUR_PI: f64 = 4.0 * std::f64::consts::PI;
