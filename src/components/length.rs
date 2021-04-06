use crate::components::{Duration, Mass};
use crate::constants::G;
use std::f64::consts::PI;

pub const M: Length = Length::in_m(1.0);
pub const KM: Length = Length::in_m(1e3);
pub const AU: Length = Length::in_m(1.495978707e11);
pub const LY: Length = Length::in_m(9.4607e15);

vector_and_scalar! {
    struct Distance([struct Length(f64); 2]) {
        fn in_m(meters) -> Self;
    }
}

impl Length {
    pub fn of_orbit(mass: Mass, period: Duration) -> Self {
        const FOUR_PI_SQUARED: f64 = 4.0 * (PI * PI);
        const ONE_THIRD: f64 = 1.0 / 3.0;
        let meters =
            (G * mass.value * period.value * period.value / FOUR_PI_SQUARED).powf(ONE_THIRD);
        Length::in_m(meters)
    }
}

impl Distance {
    /// Returns the position vector given an angle and a radius
    ///
    ///  # Arguments
    ///
    /// * `angle` - as measured clockwise from the positive y-axis
    /// * `magnitude` - length of the resulting vector
    pub fn from_angle_and_radius(angle: super::Angle, magnitude: Length) -> Self {
        let x = magnitude * angle.sin();
        let y = magnitude * angle.cos();
        Self { x, y }
    }
}

#[test]
fn vector_and_scalar() {
    let len = 4.0 * M;
    let dist = (2.0, 3.0) * M;

    assert_eq!(Length::in_m(4.0), len);
    assert_eq!(Distance::in_m(2.0, 3.0), dist)
}

#[test]
fn orbit_radius() {
    // source: https://en.wikipedia.org/wiki/Orbital_period#Small_body_orbiting_a_central_body
    let expected = Length::in_m(1.0807);
    let actual = Length::of_orbit(Mass::in_kg(100.0), Duration::in_hours(24.0));
    assert!((expected - actual).abs().value < 0.0001);
}
