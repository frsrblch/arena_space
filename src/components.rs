use std::ops::*;

macro_rules! scalar {
    ($scalar:ident, $base:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $scalar($base);

        impl $scalar {
            #[allow(dead_code)]
            fn new(value: $base) -> Self {
                Self(value)
            }
        }

        impl Add for $scalar {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl AddAssign for $scalar {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $scalar {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl SubAssign for $scalar {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul<$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: $base) -> Self {
                Self(self.0 * rhs)
            }
        }

        impl MulAssign<$base> for $scalar {
            fn mul_assign(&mut self, rhs: $base) {
                self.0 *= rhs;
            }
        }

        impl Div<$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: $base) -> Self {
                Self(self.0 / rhs)
            }
        }

        impl DivAssign<$base> for $scalar {
            fn div_assign(&mut self, rhs: $base) {
                self.0 /= rhs;
            }
        }

        impl Div for $scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Neg for $scalar {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl Rem for $scalar {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }
    };
    ($scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        scalar!($scalar, $base);

        paste::item! {
            impl $scalar {
                pub fn [<in_ $abrev>] ($unit: $base) -> Self {
                    Self($unit)
                }
            }
        }
    };
    ($scalar:ident, $unit:ident, $abrev:ident) => {
        scalar!($scalar, $unit, $abrev, f64);
    }
}

macro_rules! vector {
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        scalar!($scalar, $unit, $abrev, $base);

        paste::item! {
            #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
            pub struct $vector {
                pub x: $scalar,
                pub y: $scalar,
            }

            impl $vector {
                pub fn [<in_ $abrev>](x: $base, y: $base) -> Self {
                    Self {
                        x: $scalar(x),
                        y: $scalar(y),
                    }
                }

                pub fn magnitude(self) -> $scalar {
                    $scalar(self.magnitude_squared_float().sqrt())
                }

                fn magnitude_squared_float(self) -> $base {
                    self.x.0 * self.x.0 + self.y.0 * self.y.0
                }
            }
        }

        impl Add for $vector {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Sub for $vector {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl AddAssign for $vector {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl SubAssign for $vector {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl Mul<$base> for $vector {
            type Output = Self;
            fn mul(self, rhs: $base) -> Self {
                Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl MulAssign<$base> for $vector {
            fn mul_assign(&mut self, rhs: $base) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl DivAssign<$base> for $vector {
            fn div_assign(&mut self, rhs: $base) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl Neg for $vector {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }
    };
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident) => {
        vector!($vector, $scalar, $unit, $abrev, f64);
    }
}

macro_rules! scalar_div {
    ($num:ty, $den:ty, $res:ty) => {
        impl Div<$den> for $num {
            type Output = $res;
            fn div(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.0 / rhs.0)
            }
        }
        impl Mul<$den> for $res {
            type Output = $num;
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }
        impl Mul<$res> for $den {
            type Output = $num;
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        paste::item! {
            #[test]
            fn [<$num:snake _ $den:snake _ $res:snake _conversion_tests>] () {
                let numerator = $num::new(6.0);
                let denominator = $den::new(2.0);
                let result = $res::new(3.0);
            
                assert_eq!(result, numerator / denominator);
                assert_eq!(numerator, result * denominator);
                assert_eq!(numerator, denominator * result);                
            }
        }
    }
}

scalar!(Mass, kilograms, kg);
scalar!(Temperature, kelvin, k);
scalar!(Angle, radians, rad);

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

vector!(Pixel, PixelScalar, pixels, px, f32);
scalar!(PixelScale, meters_per_pixel, mppx, f32);


scalar!(Population, f64);

impl Population {
    pub fn in_millions(mm_people: f64) -> Self {
        Self(mm_people * 1e6)
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
    fn div(self, rhs: Self) -> f64 {
        self.0 / rhs.0
    }
}

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Time {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Duration {
        Duration::in_s(self.0 - rhs.0)
    }
}

impl Div<Duration> for Time {
    type Output = f64;
    fn div(self, rhs: Duration) -> Self::Output {
        self.0 / rhs.0
    }
}

scalar!(Duration, seconds, s);

scalar!(MassRate, kg_per_second, kgps);

scalar_div!(Mass, Duration, MassRate);