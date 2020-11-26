use super::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Fraction(f64);

impl Eq for Fraction {}

impl Into<f64> for Fraction {
    fn into(self) -> f64 {
        self.0
    }
}

impl Fraction {
    pub const fn new(value: f64) -> Self {
        if value == f64::NEG_INFINITY || value.is_nan() {
            return Self(0.0);
        }

        if value == f64::INFINITY {
            return Self(1.0);
        }

        match value {
            value if value < 0.0 => Self(0.0),
            value if value > 1.0 => Self(1.0),
            value => Self(value),
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Distribution<Fraction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fraction {
        Fraction(rng.gen_range(0.0, 1.0))
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Self(self.0 * rhs.0)
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
