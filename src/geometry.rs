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
        FOUR_PI * self.dim * self.dim
    }
}

const FOUR_PI: f64 = 4.0 * std::f64::consts::PI;
