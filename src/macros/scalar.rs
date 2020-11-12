macro_rules! scalar {
    {
        struct $scalar:ident($base:ty)
    } => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $scalar {
            pub(super) value: $base,
        }

        impl $scalar {
            #[allow(dead_code)]
            pub(super) const fn new(value: $base) -> Self {
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

        impl std::ops::Add for $scalar {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value + rhs.value)
            }
        }

        impl std::ops::Add<&$scalar> for $scalar {
            type Output = Self;
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl std::ops::Add<$scalar> for &$scalar {
            type Output = $scalar;
            fn add(self, rhs: $scalar) -> Self::Output {
                *self + rhs
            }
        }

        impl std::ops::Add for &$scalar {
            type Output = $scalar;
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl std::ops::AddAssign for $scalar {
            fn add_assign(&mut self, rhs: Self) {
                self.value += rhs.value;
            }
        }

        impl std::ops::AddAssign<&Self> for $scalar {
            fn add_assign(&mut self, rhs: &Self) {
                self.value += rhs.value;
            }
        }

        impl std::ops::Sub for $scalar {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output::new(self.value - rhs.value)
            }
        }

        impl std::ops::Sub<&$scalar> for $scalar {
            type Output = Self;
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl std::ops::Sub<$scalar> for &$scalar {
            type Output = $scalar;
            fn sub(self, rhs: $scalar) -> Self::Output {
                *self - rhs
            }
        }

        impl std::ops::Sub for &$scalar {
            type Output = $scalar;
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl std::ops::SubAssign for $scalar {
            fn sub_assign(&mut self, rhs: Self) {
                self.value -= rhs.value;
            }
        }

        impl std::ops::SubAssign<&Self> for $scalar {
            fn sub_assign(&mut self, rhs: &Self) {
                self.value -= rhs.value;
            }
        }

        impl std::ops::Mul<$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value * rhs)
            }
        }

        impl std::ops::Mul<$base> for &$scalar {
            type Output = $scalar;
            fn mul(self, rhs: $base) -> Self::Output {
                *self * rhs
            }
        }

        impl std::ops::Mul<&$base> for $scalar {
            type Output = Self;
            fn mul(self, rhs: &$base) -> Self::Output {
                self * *rhs
            }
        }

        impl std::ops::Mul<&$base> for &$scalar {
            type Output = $scalar;
            fn mul(self, rhs: &$base) -> Self::Output {
                *self * *rhs
            }
        }

        impl std::ops::Mul<$scalar> for $base {
            type Output = $scalar;
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&$scalar> for $base {
            type Output = $scalar;
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<$scalar> for &$base {
            type Output = $scalar;
            fn mul(self, rhs: $scalar) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&$scalar> for &$base {
            type Output = $scalar;
            fn mul(self, rhs: &$scalar) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<$base> for $scalar {
            fn mul_assign(&mut self, rhs: $base) {
                self.value *= rhs;
            }
        }

        impl std::ops::MulAssign<&$base> for $scalar {
            fn mul_assign(&mut self, rhs: &$base) {
                self.value *= rhs;
            }
        }

        impl std::ops::Div<$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output::new(self.value / rhs)
            }
        }

        impl std::ops::Div<$base> for &$scalar {
            type Output = $scalar;
            fn div(self, rhs: $base) -> Self::Output {
                *self / rhs
            }
        }

        impl std::ops::Div<&$base> for $scalar {
            type Output = Self;
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl std::ops::Div<&$base> for &$scalar {
            type Output = $scalar;
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl std::ops::DivAssign<$base> for $scalar {
            fn div_assign(&mut self, rhs: $base) {
                self.value /= rhs;
            }
        }

        impl std::ops::DivAssign<&$base> for $scalar {
            fn div_assign(&mut self, rhs: &$base) {
                self.value /= rhs;
            }
        }

        impl std::ops::Div for $scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl std::ops::Div<&$scalar> for $scalar {
            type Output = $base;
            fn div(self, rhs: &Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl std::ops::Div<$scalar> for &$scalar {
            type Output = $base;
            fn div(self, rhs: $scalar) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl std::ops::Div for &$scalar {
            type Output = $base;
            fn div(self, rhs: Self) -> Self::Output {
                self.value / rhs.value
            }
        }

        impl std::ops::Neg for $scalar {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::Output::new(-self.value)
            }
        }

        impl std::ops::Neg for &$scalar {
            type Output = $scalar;
            fn neg(self) -> Self::Output {
                -*self
            }
        }

        impl std::ops::Rem for $scalar {
            type Output = $scalar;
            fn rem(self, rhs: $scalar) -> Self::Output {
                Self::Output::new(self.value % rhs.value)
            }
        }

        impl std::ops::Rem<&$scalar> for $scalar {
            type Output = $scalar;
            fn rem(self, rhs: &$scalar) -> Self::Output {
                self % *rhs
            }
        }

        impl std::ops::Rem<$scalar> for &$scalar {
            type Output = $scalar;
            fn rem(self, rhs: $scalar) -> Self::Output {
                *self % rhs
            }
        }

        impl std::ops::Rem<&$scalar> for &$scalar {
            type Output = $scalar;
            fn rem(self, rhs: &$scalar) -> Self::Output {
                *self % *rhs
            }
        }

        impl std::iter::Sum<$scalar> for $scalar {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::new(iter.map(|v| v.value).sum())
            }
        }

        impl<'a> std::iter::Sum<&'a Self> for $scalar {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().sum()
            }
        }
    };
    {
        struct $scalar:ident($base:ty) {
            fn $in_unit:ident($unit:ident) -> Self;
        }
    } => {
        scalar!(struct $scalar($base));

        impl $scalar {
            pub const fn $in_unit($unit: $base) -> Self {
                Self::new($unit)
            }
        }
    };
}
