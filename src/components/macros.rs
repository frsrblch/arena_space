macro_rules! scalar {
    ($scalar:ident, $base:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $scalar { pub value: $base }

        impl $scalar {
            #[allow(dead_code)]
            const fn new(value: $base) -> Self {
                Self { value }
            }

            pub const fn zero() -> Self {
                Self::new(0.0)
            }

            pub fn min(self, rhs: Self) -> Self {
                Self::new(self.value.min(rhs.value))
            }

            pub fn max(self, rhs: Self) -> Self {
                Self::new(self.value.max(rhs.value))
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

        impl Sum<$scalar> for $scalar {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                Self::new(iter.map(|v| v.value).sum())
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