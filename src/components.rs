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

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    pub fn in_rad(radians: f64) -> Self {
        Self(radians)
    }
}