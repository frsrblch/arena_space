use std::ops::*;

macro_rules! scalar {
    ($scalar:ident, $base:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $scalar { pub value: $base }

        impl $scalar {
            #[allow(dead_code)]
            const fn new(value: $base) -> Self {
                Self { value }
            }
        }

        impl Add for $scalar {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.value + rhs.value)
            }
        }

        impl AddAssign for $scalar {
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl Sub for $scalar {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.value - rhs.value)
            }
        }

        impl SubAssign for $scalar {
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl Mul<$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: $base) -> Self {
                Self::new(self.value * rhs)
            }
        }

        impl Mul<$scalar> for $base {
            type Output = $scalar;
            fn mul(self, rhs: $scalar) -> Self::Output {
                $scalar::new(self * rhs.value)
            }
        }

        impl MulAssign<$base> for $scalar {
            fn mul_assign(&mut self, rhs: $base) {
                self.value *= rhs;
            }
        }

        impl Div<$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: $base) -> Self {
                Self::new(self.value / rhs)
            }
        }

        impl DivAssign<$base> for $scalar {
            fn div_assign(&mut self, rhs: $base) {
                self.value /= rhs;
            }
        }

        impl Div for $scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl Neg for $scalar {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::new(-self.value)
            }
        }

        impl Rem for $scalar {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self {
                Self::new(self.value % rhs.value)
            }
        }
    };
    ($scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        scalar!($scalar, $base);

        paste::item! {
            impl $scalar {
                pub const fn [<in_ $abrev>] ($unit: $base) -> Self {
                    Self::new($unit)
                }
            }
        }
    };
    ($scalar:ident, $unit:ident, $abrev:ident) => {
        scalar!($scalar, $unit, $abrev, f64);
    }
}

macro_rules! vector {
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident) => {
        vector!($vector, $scalar, $unit, $abrev, f64);
    };
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $vector {
            pub x: $scalar,
            pub y: $scalar,
        }

        paste::item! {
            impl $vector {
                pub const fn [<in_ $abrev>](x: $base, y: $base) -> Self {
                    Self {
                        x: $scalar::new(x),
                        y: $scalar::new(y),
                    }
                }

                pub fn magnitude(self) -> $scalar {
                    $scalar::new(self.magnitude_squared_float().sqrt())
                }

                fn magnitude_squared_float(self) -> $base {
                    self.x.value * self.x.value + self.y.value * self.y.value
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
}

macro_rules! vector_and_scalar {
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        scalar!($scalar, $unit, $abrev, $base);
        vector!($vector, $scalar, $unit, $abrev, $base);
    };
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident) => {
        vector_and_scalar!($vector, $scalar, $unit, $abrev, f64);
    }
}

macro_rules! scalar_div {
    ($num:ty, $den:ty, $res:ty) => {
        impl Div<$den> for $num {
            type Output = $res;
            fn div(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value / rhs.value)
            }
        }
        impl Mul<$den> for $res {
            type Output = $num;
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }
        impl Mul<$res> for $den {
            type Output = $num;
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: Length,
    pub y: Length,
}

impl Position {
    pub const fn in_m(x: f64, y: f64) -> Self {
        Self { 
            x: Length::in_m(x),
            y: Length::in_m(y)
        }
    }

    pub fn in_ly(x: f64, y: f64) -> Self {
        Self::in_m(x * Self::M_PER_LY, y * Self::M_PER_LY)
    }

    const M_PER_LY: f64 = 9.460_730_472_580_800e15;
}

impl From<Distance> for Position {
    fn from(value: Distance) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Add<Distance> for Position {
    type Output = Position;
    fn add(self, rhs: Distance) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }    
}

impl AddAssign<Distance> for Position {
    fn add_assign(&mut self, rhs: Distance) {
        self.x += rhs.x;
        self.y += rhs.y;
    }    
}

impl Sub<Distance> for Position {
    type Output = Position;
    fn sub(self, rhs: Distance) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }    
}

impl SubAssign<Distance> for Position {
    fn sub_assign(&mut self, rhs: Distance) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }    
}

impl Sub for Position {
    type Output = Distance;
    fn sub(self, rhs: Self) -> Distance {
        Distance {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

scalar!(PixelScale, meters_per_pixel, m_per_px, f32);

scalar!(Population, f64);

impl Population {
    pub fn in_millions(mm_people: f64) -> Self {
        Self::new(mm_people * 1e6)
    }
}

/// Elapsed game time in seconds. Distinct from Duration, which is a relative amount of time.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time { pub value: f64 }

impl Time {
    pub fn in_s(seconds: f64) -> Self {
        Self::new(seconds)
    }

    fn new(value: f64) -> Self {
        Self { value }
    }

    pub const SECONDS_PER_DAY: f64 = 3600.0 * 24.0;
}

impl Div for Time {
    type Output = f64;
    fn div(self, rhs: Self) -> f64 {
        self.value / rhs.value
    }
}

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        Self::new(self.value + rhs.value)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        self.value += rhs.value;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self {
        Self::new(self.value - rhs.value)
    }
}

impl Sub for Time {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Duration {
        Duration::in_s(self.value - rhs.value)
    }
}

impl Div<Duration> for Time {
    type Output = f64;
    fn div(self, rhs: Duration) -> Self::Output {
        self.value / rhs.value
    }
}

scalar!(Duration, seconds, s);

scalar!(MassRate, kg_per_second, kg_per_s);

scalar_div!(Mass, Duration, MassRate);