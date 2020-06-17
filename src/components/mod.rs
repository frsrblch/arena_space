use std::fmt::{Display, Result, Formatter};
use std::ops::*;

pub use position::*;
mod position;

pub use time::*;
mod time;

pub use orbit::*;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::iter::Sum;

mod orbit;

#[macro_use]
mod macros;

scalar!(Mass, kilograms, kg);

impl Display for Mass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:.0} kg", self.value)
    }
}

#[test]
fn mass_display() {
    assert_eq!("25 kg", Mass::in_kg(25.0).to_string());
}

impl Mass {
    pub fn tons(self) -> Tons {
        Tons(self)
    }
}

pub struct Tons(Mass);

impl Display for Tons {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:.0} t", self.0.value / 1e3)
    }
}

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

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Fraction(f64);

impl Into<f64> for Fraction {
    fn into(self) -> f64 {
        self.0
    }
}

impl Fraction {
    pub fn new(value: f64) -> Self {
        if value == f64::NEG_INFINITY || value == f64::NAN {
            return Self(0.0)
        }

        if value == f64::INFINITY {
            return Self(1.0)
        }

        match value {
            value if value < 0.0 => Self(0.0),
            value if value > 1.0 => Self(1.0),
            value => Self(value)
        }
    }
}

impl Distribution<Fraction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fraction {
        Fraction(rng.gen_range(0.0, 1.0))
    }
}

impl Mul<f64> for Fraction {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<Fraction> for f64 {
    type Output = f64;

    fn mul(self, rhs: Fraction) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod test;