use std::fmt::{Display, Formatter, Result};
use std::ops::*;

use num_format::{Locale, ToFormattedString};

pub use self::angle::*;
pub use self::area::*;
pub use self::credits::*;
pub use self::economy::*;
pub use self::ema::*;
pub use self::fraction::*;
pub use self::length::*;
pub use self::mass::*;
pub use self::mass_rate::*;
pub use self::orbit::*;
pub use self::population::*;
pub use self::position::*;
pub use self::speed::*;
pub use self::time::*;

mod angle;
mod area;
mod credits;
mod economy;
mod ema;
mod fraction;
mod length;
mod mass;
mod mass_rate;
mod orbit;
mod population;
mod position;
mod speed;
mod time;

scalar! {
    struct Temperature(f64) {
        fn in_k(kelvin) -> Self;
    }
}

scalar! {
    struct PixelScale(f32) {
        fn in_m_per_px(meters_per_pixel) -> Self;
    }
}

#[cfg(test)]
mod test;
