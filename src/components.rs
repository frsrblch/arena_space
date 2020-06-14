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

        impl Into<$base> for $scalar {
            fn into(self) -> $base {
                self.value
            }
        }

        impl From<$base> for $scalar {
            fn from(value: $base) -> Self {
                Self::new(value)
            }
        }

        impl Add for $scalar {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value + rhs.value)
            }
        }

        impl Add<&$scalar> for $scalar {
            type Output = Self;
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl Add<$scalar> for &$scalar {
            type Output = $scalar;
            fn add(self, rhs: $scalar) -> Self::Output {
                *self + rhs
            }
        }

        impl Add for &$scalar {
            type Output = $scalar;
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl AddAssign for $scalar {
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl AddAssign<&Self> for $scalar {
            fn add_assign(&mut self, rhs: &Self) {
                self.value += rhs.value;
            }
        }

        impl Sub for $scalar {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value - rhs.value)
            }
        }

        impl Sub<&$scalar> for $scalar {
            type Output = Self;
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl Sub<$scalar> for &$scalar {
            type Output = $scalar;
            fn sub(self, rhs: $scalar) -> Self::Output {
                *self - rhs
            }
        }

        impl Sub for &$scalar {
            type Output = $scalar;
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl SubAssign for $scalar {
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl SubAssign<&Self> for $scalar {
            fn sub_assign(&mut self, rhs: &Self) {
                self.value -= rhs.value;
            }
        }

        impl Mul<$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value * rhs)
            }
        }

        impl Mul<$base> for &$scalar {
            type Output = $scalar;
            fn mul(self, rhs: $base) -> Self::Output {
                *self * rhs
            }
        }

        impl Mul<&$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: &$base) -> Self::Output {
                self * *rhs
            }
        }

        impl Mul<&$base> for &$scalar {
            type Output = $scalar;
            fn mul(self, rhs: &$base) -> Self::Output {
                *self * *rhs
            }
        }

        impl Mul<$scalar> for $base {
            type Output = $scalar;
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<&$scalar> for $base {
            type Output = $scalar;
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<$scalar> for &$base {
            type Output = $scalar;
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<&$scalar> for &$base {
            type Output = $scalar;
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$base> for $scalar {
            fn mul_assign(&mut self, rhs: $base) {
                self.value *= rhs;
            }
        }

        impl MulAssign<&$base> for $scalar {
            fn mul_assign(&mut self, rhs: &$base) {
                self.value *= rhs;
            }
        }

        impl Div<$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value / rhs)
            }
        }

        impl Div<$base> for &$scalar {
            type Output = $scalar;
            fn div(self, rhs: $base) -> Self::Output {
                *self / rhs
            }
        }

        impl Div<&$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl Div<&$base> for &$scalar {
            type Output = $scalar;
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl DivAssign<$base> for $scalar {
            fn div_assign(&mut self, rhs: $base) {
                self.value /= rhs;
            }
        }

        impl DivAssign<&$base> for $scalar {
            fn div_assign(&mut self, rhs: &$base) {
                self.value /= rhs;
            }
        }

        impl Div for $scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl Div<&$scalar> for $scalar {
            type Output = $base;
            fn div(self, rhs: &Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl Div<$scalar> for &$scalar {
            type Output = $base;
            fn div(self, rhs: $scalar) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl Div for &$scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl Neg for $scalar {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::Output::new(-self.value)
            }
        }

        impl Neg for &$scalar {
            type Output = $scalar;
            fn neg(self) -> Self::Output {
                -*self
            }
        }

        impl Rem for $scalar {
            type Output = $scalar;
            fn rem(self, rhs: $scalar) -> Self::Output {
                Self::Output::new(self.value % rhs.value)
            }
        }

        impl Rem<&$scalar> for $scalar {
            type Output = $scalar;
            fn rem(self, rhs: &$scalar) -> Self::Output {
                self % *rhs
            }
        }

        impl Rem<$scalar> for &$scalar {
            type Output = $scalar;
            fn rem(self, rhs: $scalar) -> Self::Output {
                *self % rhs
            }
        }

        impl Rem<&$scalar> for &$scalar {
            type Output = $scalar;
            fn rem(self, rhs: &$scalar) -> Self::Output {
                *self % *rhs
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

        impl Add<$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Add<$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Add<&$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Add<&$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Sub<$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl Sub<$vector> for &$vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl Sub<&$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl Sub<&$vector> for &$vector {
            type Output = $vector;
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl AddAssign<$vector> for $vector {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl AddAssign<&$vector> for $vector {
            fn add_assign(&mut self, rhs: &$vector) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl SubAssign<$vector> for $vector {
            fn sub_assign(&mut self, rhs: $vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl SubAssign<&$vector> for $vector {
            fn sub_assign(&mut self, rhs: &$vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl Mul<$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl Mul<$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl Mul<&$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl Mul<&$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl Mul<$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<$vector> for &$base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<&$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl Mul<&$vector> for &$base {
            type Output = $vector;
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl MulAssign<$base> for $vector {
            fn mul_assign(&mut self, rhs: $base) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl MulAssign<&$base> for $vector {
            fn mul_assign(&mut self, rhs: &$base) {
                *self *= *rhs;
            }
        }

        impl Div<$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl Div<$base> for &$vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl Div<&$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl Div<&$base> for &$vector {
            type Output = $vector;
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl DivAssign<$base> for $vector {
            fn div_assign(&mut self, rhs: $base) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl DivAssign<&$base> for $vector {
            fn div_assign(&mut self, rhs: &$base) {
                *self /= *rhs;
            }
        }

        impl Neg for $vector {
            type Output = $vector;
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl Neg for &$vector {
            type Output = $vector;
            fn neg(self) -> Self::Output {
                Self::Output {
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
        
        impl Div<$den> for &$num {
            type Output = $res;
            fn div(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value / rhs.value)
            }
        }
        impl Div<&$den> for $num {
            type Output = $res;
            fn div(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value / rhs.value)
            }
        }
        
        impl Div<&$den> for &$num {
            type Output = $res;
            fn div(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value / rhs.value)
            }
        }


        impl Mul<$den> for $res {
            type Output = $num;
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<$den> for &$res {
            type Output = $num;
            fn mul(self, rhs: $den) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<&$den> for $res {
            type Output = $num;
            fn mul(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<&$den> for &$res {
            type Output = $num;
            fn mul(self, rhs: &$den) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }


        impl Mul<$res> for $den {
            type Output = $num;
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<$res> for &$den {
            type Output = $num;
            fn mul(self, rhs: $res) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<&$res> for $den {
            type Output = $num;
            fn mul(self, rhs: &$res) -> Self::Output {
                Self::Output::new(self.value * rhs.value)
            }
        }

        impl Mul<&$res> for &$den {
            type Output = $num;
            fn mul(self, rhs: &$res) -> Self::Output {
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

impl Duration {
    pub fn in_days(days: f64) -> Self {
        Self::in_s(days * Self::SECONDS_PER_DAY)
    }

    pub const SECONDS_PER_DAY: f64 = 3600.0 * 24.0;
}

scalar!(MassRate, kg_per_second, kg_per_s);
scalar_div!(Mass, Duration, MassRate);

scalar!(MassRatePerPerson, kg_per_person_second, kg_per_s_person);
scalar_div!(MassRate, Population, MassRatePerPerson);

#[cfg(test)]
mod tests {
    use super::*;

    vector_and_scalar!(TestVector, TestScalar, test, test);

    #[test]
    fn scalar_add_tests() {
        let a = TestScalar::in_test(2.0);
        let b = TestScalar::in_test(3.0);

        let expected = TestScalar::in_test(5.0);

        assert_eq!(expected,  a +  b);
        assert_eq!(expected, &a +  b);
        assert_eq!(expected,  a + &b);
        assert_eq!(expected, &a + &b);
    }

    #[test]
    fn scalar_add_assign_tests() {
        let a_0 = TestScalar::in_test(2.0);
        let b = TestScalar::in_test(3.0);

        let expected = TestScalar::in_test(5.0);

        let mut a = a_0;
        a += b;
        assert_eq!(expected,  a);

        let mut a = a_0;
        a += &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn scalar_sub_tests() {
        let a = TestScalar::in_test(2.0);
        let b = TestScalar::in_test(3.0);

        let expected = TestScalar::in_test(-1.0);

        assert_eq!(expected,  a -  b);
        assert_eq!(expected, &a -  b);
        assert_eq!(expected,  a - &b);
        assert_eq!(expected, &a - &b);
    }

    #[test]
    fn scalar_sub_assign_tests() {
        let a_0 = TestScalar::in_test(2.0);
        let b = TestScalar::in_test(3.0);

        let expected = TestScalar::in_test(-1.0);

        let mut a = a_0;
        a -= b;
        assert_eq!(expected,  a);

        let mut a = a_0;
        a -= &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn scalar_mul_test() {
        let a = TestScalar::in_test(2.0);
        let b = 3.0f64;

        let expected = TestScalar::in_test(6.0);

        assert_eq!(expected,  a *  b);
        assert_eq!(expected, &a *  b);
        assert_eq!(expected,  a * &b);
        assert_eq!(expected, &a * &b);

        assert_eq!(expected,  b *  a);
        assert_eq!(expected, &b *  a);
        assert_eq!(expected,  b * &a);
        assert_eq!(expected, &b * &a);
    }

    #[test]
    fn scalar_mul_assign_test() {
        let a_0 = TestScalar::in_test(2.0);
        let b = 3.0f64;

        let expected = TestScalar::in_test(6.0);

        let mut a = a_0;
        a *= b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a *= &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn scalar_div_test() {
        let a = TestScalar::in_test(2.0);
        let b = 3.0f64;

        let expected = TestScalar::in_test(2.0 / 3.0);

        assert_eq!(expected,  a /  b);
        assert_eq!(expected, &a /  b);
        assert_eq!(expected,  a / &b);
        assert_eq!(expected, &a / &b);
    }

    #[test]
    fn scalar_div_assign_test() {
        let a_0 = TestScalar::in_test(6.0);
        let b = 3.0f64;

        let expected = TestScalar::in_test(2.0);

        let mut a = a_0;
        a /= b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a /= &b;
        assert_eq!(expected, a);
    }

    scalar!(Num, test, test);
    scalar!(Den, test, test);
    scalar!(Res, test, test);

    scalar_div!(Num, Den, Res); // Num / Den = Res

    #[test]
    fn scalar_div_conversion_test() {
        let num = Num::in_test(6.0);
        let den = Den::in_test(2.0);
        let res = Res::in_test(3.0);

        assert_eq!(res,  num /  den);
        assert_eq!(res, &num /  den);
        assert_eq!(res,  num / &den);
        assert_eq!(res, &num / &den);
    }

    #[test]
    fn scalar_mul_conversion_test() {
        let num = Num::in_test(6.0);
        let den = Den::in_test(2.0);
        let res = Res::in_test(3.0);

        assert_eq!(num,  res *  den);
        assert_eq!(num, &res *  den);
        assert_eq!(num,  res * &den);
        assert_eq!(num, &res * &den);

        assert_eq!(num,  den *  res);
        assert_eq!(num, &den *  res);
        assert_eq!(num,  den * &res);
        assert_eq!(num, &den * &res);
    }

    #[test]
    fn scalar_rem_test() {
        let a = TestScalar::in_test(5.0);
        let b = TestScalar::in_test(3.0);

        let rem = TestScalar::in_test(2.0);

        assert_eq!(rem,  a %  b);
        assert_eq!(rem,  a % &b);
        assert_eq!(rem, &a %  b);
        assert_eq!(rem, &a % &b);
    }

    #[test]
    fn scalar_neg_test() {
        let a = TestScalar::in_test(2.0);

        let neg = TestScalar::in_test(-2.0);

        assert_eq!(neg, -&a);
        assert_eq!(neg, - a);
    }

    #[test]
    fn vector_add_test() {
        let a = TestVector::in_test(2.0, 3.0);
        let b = TestVector::in_test(5.0, 7.0);

        let expected = TestVector::in_test(7.0, 10.0);

        assert_eq!(expected,  a +  b);
        assert_eq!(expected, &a +  b);
        assert_eq!(expected,  a + &b);
        assert_eq!(expected, &a + &b);
    }

    #[test]
    fn vector_add_assign_test() {
        let a_0 = TestVector::in_test(2.0, 3.0);
        let b = TestVector::in_test(5.0, 7.0);

        let expected = TestVector::in_test(7.0, 10.0);

        let mut a = a_0;
        a += b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a += &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn vector_sub_test() {
        let a = TestVector::in_test(2.0, 3.0);
        let b = TestVector::in_test(5.0, 7.0);

        let expected = TestVector::in_test(-3.0, -4.0);

        assert_eq!(expected,  a -  b);
        assert_eq!(expected, &a -  b);
        assert_eq!(expected,  a - &b);
        assert_eq!(expected, &a - &b);
    }

    #[test]
    fn vector_sub_assign_test() {
        let a_0 = TestVector::in_test(2.0, 3.0);
        let b = TestVector::in_test(5.0, 7.0);

        let expected = TestVector::in_test(-3.0, -4.0);

        let mut a = a_0;
        a -= b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a -= &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn vector_mul_test() {
        let a = TestVector::in_test(2.0, 3.0);
        let b = 5.0f64;

        let expected = TestVector::in_test(10.0, 15.0);

        assert_eq!(expected,  a *  b);
        assert_eq!(expected, &a *  b);
        assert_eq!(expected,  a * &b);
        assert_eq!(expected, &a * &b);

        assert_eq!(expected,  b *  a);
        assert_eq!(expected, &b *  a);
        assert_eq!(expected,  b * &a);
        assert_eq!(expected, &b * &a);
    }

    #[test]
    fn vector_mul_assign_test() {
        let a_0 = TestVector::in_test(2.0, 3.0);
        let b = 5.0f64;

        let expected = TestVector::in_test(10.0, 15.0);

        let mut a = a_0;
        a *= b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a *= &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn vector_div_test() {
        let a = TestVector::in_test(10.0, 15.0);
        let b = 5.0f64;

        let expected = TestVector::in_test(2.0, 3.0);

        assert_eq!(expected,  a /  b);
        assert_eq!(expected, &a /  b);
        assert_eq!(expected,  a / &b);
        assert_eq!(expected, &a / &b);
    }

    #[test]
    fn vector_div_assign_test() {
        let a_0 = TestVector::in_test(10.0, 15.0);
        let b = 5.0f64;

        let expected = TestVector::in_test(2.0, 3.0);

        let mut a = a_0;
        a /= b;
        assert_eq!(expected, a);

        let mut a = a_0;
        a /= &b;
        assert_eq!(expected, a);
    }

    #[test]
    fn vector_neg_test() {
        let a = TestVector::in_test(2.0, 3.0);
        
        let neg = TestVector::in_test(-2.0, -3.0);

        assert_eq!(neg, - a);
        assert_eq!(neg, -&a);
    }
}