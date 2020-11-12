macro_rules! vector {
    ($vector:ident, $scalar:ident, $unit:ident, $in_unit:ident) => {
        vector!($vector, $scalar, $unit, $in_unit, f64);
    };
    ($vector:ident, $scalar:ident, $unit:ident, $in_unit:ident, $base:ty) => {
        #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
        pub struct $vector {
            pub x: $scalar,
            pub y: $scalar,
        }

        impl $vector {
            pub const fn $in_unit(x: $base, y: $base) -> Self {
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

        impl std::ops::Add<$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl std::ops::Add<$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl std::ops::Add<&$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl std::ops::Add<&$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl std::ops::Sub<$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl std::ops::Sub<$vector> for &$vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl std::ops::Sub<&$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl std::ops::Sub<&$vector> for &$vector {
            type Output = $vector;
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl std::ops::AddAssign<$vector> for $vector {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl std::ops::AddAssign<&$vector> for $vector {
            fn add_assign(&mut self, rhs: &$vector) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl std::ops::SubAssign<$vector> for $vector {
            fn sub_assign(&mut self, rhs: $vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl std::ops::SubAssign<&$vector> for $vector {
            fn sub_assign(&mut self, rhs: &$vector) {
                self.x -= rhs.x;
                self.y -= rhs.y;
            }
        }

        impl std::ops::Mul<$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl std::ops::Mul<$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl std::ops::Mul<&$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl std::ops::Mul<&$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl std::ops::Mul<$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<$vector> for &$base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Mul<&$vector> for &$base {
            type Output = $vector;
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::MulAssign<$base> for $vector {
            fn mul_assign(&mut self, rhs: $base) {
                self.x *= rhs;
                self.y *= rhs;
            }
        }

        impl std::ops::MulAssign<&$base> for $vector {
            fn mul_assign(&mut self, rhs: &$base) {
                *self *= *rhs;
            }
        }

        impl std::ops::Div<$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl std::ops::Div<$base> for &$vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl std::ops::Div<&$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl std::ops::Div<&$base> for &$vector {
            type Output = $vector;
            fn div(self, rhs: &$base) -> Self::Output {
                *self / *rhs
            }
        }

        impl std::ops::DivAssign<$base> for $vector {
            fn div_assign(&mut self, rhs: $base) {
                self.x /= rhs;
                self.y /= rhs;
            }
        }

        impl std::ops::DivAssign<&$base> for $vector {
            fn div_assign(&mut self, rhs: &$base) {
                *self /= *rhs;
            }
        }

        impl std::ops::Neg for $vector {
            type Output = $vector;
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl std::ops::Neg for &$vector {
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
