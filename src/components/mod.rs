use std::ops::*;

pub use position::*;
mod position;

pub use time::*;
mod time;

pub use orbit::*;
mod orbit;

#[macro_use]
mod macros;

scalar!(Mass, kilograms, kg);

scalar!(Temperature, kelvin, k);

scalar!(Angle, radians, rad);

impl Angle {
    pub fn in_deg(degrees: f64) -> Self {
        Self::new(degrees * Self::RAD_PER_DEG)
    }

    pub fn sin(self) -> f64 {
        self.value.sin()
    }

    pub fn cos(self) -> f64 {
        self.value.cos()
    }

    const RAD_PER_DEG: f64 = std::f64::consts::PI / 180.0;
}

vector_and_scalar!(Distance, Length, meters, m);

impl Distance {
    /// Returns the position vector given an angle and a radius
    ///
    ///  # Arguments
    ///
    /// * `angle` - as measured clockwise from the positive y-axis
    /// * `magnitude` - length of the resulting vector
    pub fn from_angle_and_radius(angle: Angle, magnitude: Length) -> Self {
        let x = magnitude * angle.sin();
        let y = magnitude * angle.cos();
        Self { x, y }
    }
}

scalar!(PixelScale, meters_per_pixel, m_per_px, f32);

scalar!(Population, f64);

impl Population {
    pub fn in_millions(mm_people: f64) -> Self {
        Self::new(mm_people * 1e6)
    }
}


scalar!(Duration, seconds, s);

impl Duration {
    pub fn in_days(days: f64) -> Self {
        Self::in_s(days * Self::SECONDS_PER_DAY)
    }

    pub const SECONDS_PER_DAY: f64 = 3600.0 * 24.0;
}

scalar!(MassRate, kg_per_second, kg_per_s);
scalar_div!(Mass, Duration, MassRate);

scalar!(MassRatePerPerson, kg_per_person_second, kg_per_s_person);
scalar_div!(MassRate, Population, MassRatePerPerson);

#[derive(Debug, Copy, Clone)]
pub enum Surface {
    Barren,
    Gaseous,
    Continental,
    Volcanic,
    Oceanic,
}

#[cfg(test)]
mod test;