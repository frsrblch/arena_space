use crate::components::{Area, Length, Squared};
use std::f64::consts::PI;

pub struct Sphere {
    radius: Length,
}

impl Sphere {
    pub const fn with_radius(radius: Length) -> Self {
        Sphere { radius }
    }

    pub const fn get_area(&self) -> Area {
        4.0 * PI * self.radius.squared()
    }
}
