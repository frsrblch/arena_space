use std::ops::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mass(f64);

impl Mass {
    pub fn in_kg(kilograms: f64) -> Self {
        Self(kilograms)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Length(f64);

impl Length {
    pub fn in_m(meters: f64) -> Self {
        Self(meters)
    }
}

impl Add for Length {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::in_m(self.0 + rhs.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Temperature(f64);

impl Temperature {
    pub fn in_k(kelvin: f64) -> Self {
        Self(kelvin)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time(f64);

impl Time {
    pub fn in_s(seconds: f64) -> Self {
        Self(seconds)
    }
}

impl Div for Time {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    pub fn in_rad(radians: f64) -> Self {
        Self(radians)
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::in_rad(self.0 + rhs.0)
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::in_rad(self.0 - rhs.0)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: Length,
    pub y: Length
}

impl Position {
    pub fn in_m(x: f64, y: f64) -> Self {
        Self { x: Length::in_m(x), y: Length::in_m(y) }
    }

    /// Returns the position given an angle and a radius
    ///
    ///  # Arguments
    ///
    /// * `angle` - An angle as measured clockwise from the positive y-axis
    /// * `length` - The length of the resulting vector
    pub fn from_angle_and_radius(angle: Angle, length: Length) -> Self {
        let x = angle.0.sin() * length.0;
        let y = angle.0.cos() * length.0;
        Self::in_m(x, y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}