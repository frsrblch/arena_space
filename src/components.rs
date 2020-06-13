use std::ops::*;

// TODO write scalar and vector component macros

// TODO add ops to vector!();

macro_rules! scalar {
    ($scalar:ident, $unit:ident, $abrev:ident) => {
        paste::item! {
            #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
            pub struct $scalar(f64);

            impl $scalar {
                pub fn [<in_ $abrev>] ($unit: f64) -> Self {
                    Self($unit)
                }
            }

            impl Add for $scalar {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0 + rhs.0)
                }
            }

            impl Sub for $scalar {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self(self.0 - rhs.0)
                }
            }

            impl Mul<f64> for $scalar {
                type Output = Self;
                fn mul(self, rhs: f64) -> Self {
                    Self(self.0 * rhs)
                }
            }

            impl Div<f64> for $scalar {
                type Output = Self;
                fn div(self, rhs: f64) -> Self {
                    Self(self.0 / rhs)
                }
            }

            impl Div for $scalar {
                type Output = f64;
                fn div(self, rhs: Self) -> Self::Output {
                    self.0 / rhs.0
                }
            }
        }
    }
}

scalar!(Mass, kilograms, kg);
scalar!(Length, meters, m);
scalar!(Temperature, kelvin, k);
scalar!(Time, seconds, s);
scalar!(Angle, radians, rad);

macro_rules! vector {
    ($vector:ident, $scalar:ty, $units:ident, $abrev:ident) => {
        paste::item! {
            #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
            pub struct $vector {
                pub x: $scalar,
                pub y: $scalar,
            }

            impl $vector {
                pub fn [<in_ $abrev>](x: f64, y: f64) -> Self {
                    Self {
                        x: $scalar(x),
                        y: $scalar(y),
                    }
                }
            }
        }
    }
}

impl Angle {
    pub fn in_deg(degrees: f64) -> Self {
        Self(degrees * Self::RAD_PER_DEG)
    }

    pub fn sin(self) -> f64 {
        self.0.sin()
    }

    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    const RAD_PER_DEG: f64 = std::f64::consts::PI / 180.0;
}

vector!(Position, Length, meters, m);

impl Position {
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

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Population(f64);

impl Population {
    pub fn in_millions(mm_people: f64) -> Self {
        Self(mm_people * 1e6)
    }
}