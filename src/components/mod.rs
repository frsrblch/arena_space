use std::fmt::{Display, Formatter, Result};
use std::ops::*;

use num_format::{Locale, ToFormattedString};

pub use self::accel::*;
pub use self::angle::*;
pub use self::area::*;
pub use self::economy::*;
pub use self::ema::*;
pub use self::energy::*;
pub use self::force::*;
pub use self::fraction::*;
pub use self::length::*;
pub use self::mass::*;
pub use self::mass_rate::*;
pub use self::orbit::*;
pub use self::pixel::*;
pub use self::population::*;
pub use self::position::*;
pub use self::power::*;
pub use self::speed::*;
pub use self::temperature::*;
pub use self::time::*;

mod accel;
mod angle;
mod area;
mod economy;
mod ema;
mod energy;
mod force;
mod fraction;
mod length;
mod mass;
mod mass_rate;
mod orbit;
mod pixel;
mod population;
mod position;
mod power;
mod speed;
mod temperature;
mod time;

#[cfg(test)]
mod test;

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f64 {
    type Output = f64;
    fn sqrt(self) -> Self::Output {
        self.sqrt()
    }
}

pub trait Squared {
    type Output;
    fn squared(self) -> Self::Output;
}

impl Squared for f64 {
    type Output = Self;
    fn squared(self) -> Self::Output {
        self * self
    }
}

pub trait Wrapper: Copy {
    type Inner;
    fn value(self) -> Self::Inner;
}
