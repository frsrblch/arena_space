use std::fmt::{Display, Result, Formatter};
use std::ops::*;

pub use position::*;
mod position;

pub use self::time::*;
mod time;

pub use orbit::*;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::iter::Sum;
use chrono::Duration;

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

scalar!(Area, square_meters, m2);

impl Area {
    pub fn in_square_km(value: f64) -> Self {
        Self::in_m2(value * 1e6)
    }
}

impl Mul<Length> for Length {
    type Output = Area;
    fn mul(self, rhs: Self) -> Self::Output {
        Area::new(self.value * rhs.value)
    }
}

scalar!(PixelScale, meters_per_pixel, m_per_px, f32);

scalar!(Population, f64);

impl Population {
    pub const fn in_millions(mm_people: f64) -> Self {
        Self::new(mm_people * 1e6)
    }

    pub fn get_food_requirement(&self) -> MassRate {
        self * Self::FOOD_PER_PERSON
    }

    /// 2 kg per person per day
    const FOOD_PER_PERSON: MassRatePerPerson = MassRatePerPerson::in_kg_per_s_person(
        2.0 / DurationFloat::SECONDS_PER_DAY
    );

    pub fn millions(self) -> Millions {
        Millions(self)
    }
}

pub struct Millions(Population);

impl Display for Millions {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.0}M", self.0.value / 1e6)
    }
}

scalar!(PopulationDensity, f64);

impl PopulationDensity {
    pub const fn in_people_per_square_km(value: f64) -> Self {
        Self::new(value / 1e6)
    }
}

scalar_div!(Population, Area, PopulationDensity);

scalar!(DurationFloat, seconds, s);

impl DurationFloat {
    pub fn in_days(days: f64) -> Self {
        Self::in_s(days * Self::SECONDS_PER_DAY)
    }

    pub const SECONDS_PER_DAY: f64 = 3600.0 * 24.0;
}

impl From<chrono::Duration> for DurationFloat {
    fn from(duration: Duration) -> Self {
        let seconds = duration.num_milliseconds() as f64 / 1e3;
        DurationFloat::in_s(seconds)
    }
}

#[test]
fn duration_float_from_duration() {
    let one_second = chrono::Duration::seconds(1);
    let one_second = DurationFloat::from(one_second);

    assert_eq!(DurationFloat::in_s(1.0), one_second);
}

scalar!(MassRate, kg_per_second, kg_per_s);

impl MassRate {
    pub fn tons_per_day(self) -> TonsPerDay {
        TonsPerDay(self)
    }
}

pub struct TonsPerDay(MassRate);

impl Display for TonsPerDay {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:.0} t/day", self.0.value)
    }
}

scalar_div!(Mass, DurationFloat, MassRate);

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
        if value == f64::NEG_INFINITY || value.is_nan() {
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

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Eq for Fraction {

}

impl Distribution<Fraction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fraction {
        Fraction(rng.gen_range(0.0, 1.0))
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Self::new(self.0 * rhs.0)
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