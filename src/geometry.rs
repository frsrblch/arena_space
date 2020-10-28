use crate::components::{Area, Length};

pub struct Sphere {
    radius: Length,
}

impl Sphere {
    pub fn with_radius(radius: Length) -> Self {
        Sphere { radius }
    }

    pub fn get_area(&self) -> Area {
        let r_squared = self.radius * self.radius;
        FOUR_PI * r_squared
    }
}

const FOUR_PI: f64 = 4.0 * std::f64::consts::PI;
