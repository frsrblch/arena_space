use super::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct Fraction(f64);

impl Eq for Fraction {}

impl From<Fraction> for f64 {
    fn from(fraction: Fraction) -> Self {
        fraction.0
    }
}

impl From<f64> for Fraction {
    fn from(value: f64) -> Self {
        Self::clamp(value)
    }
}

impl Fraction {
    pub const ZERO: Fraction = Fraction::clamp(0.0);
    pub const ONE: Fraction = Fraction::clamp(1.0);

    pub const fn clamp(value: f64) -> Self {
        debug_assert!(value.is_finite());

        match value {
            value if value < 0.0 => Self(0.0),
            value if value > 1.0 => Self(1.0),
            value => Self(value),
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn powf(self, n: f64) -> f64 {
        self.0.powf(n)
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

impl Sqrt for Fraction {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Fraction(self.0.sqrt())
    }
}

impl Squared for Fraction {
    type Output = Self;

    fn squared(self) -> Self::Output {
        Fraction(self.0.squared())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! fraction_tests {
    { $( $name:ident ( $value:expr, $expected:expr ); )* } => {

        $(
            #[test]
            fn $name () {
                assert_eq!(Fraction::clamp($value).value(), $expected);
            }
        )*
    }
}

    fraction_tests! {
        zero(0.0, 0.0);
        one(1.0, 1.0);
        two(2.0, 1.0);
        neg(-1.0, 0.0);
        valid(0.4, 0.4);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn fraction_inf() {
        Fraction::clamp(f64::INFINITY);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn fraction_neg_inf() {
        Fraction::clamp(f64::NEG_INFINITY);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn fraction_nan() {
        Fraction::clamp(f64::NEG_INFINITY);
    }
}
