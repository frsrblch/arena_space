use crate::components::{Area, Length, Squared};

pub struct Sphere {
    radius: Length,
}

impl Sphere {
    pub const fn with_radius(radius: Length) -> Self {
        Sphere { radius }
    }

    pub const fn get_area(&self) -> Area {
        FOUR_PI * self.radius.squared()
    }
}

const FOUR_PI: f64 = 4.0 * std::f64::consts::PI;
