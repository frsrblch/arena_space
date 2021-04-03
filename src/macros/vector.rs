macro_rules! vector {
    {
        struct $vector:ident([$scalar:ident; 2]) {
            fn $in_unit:ident($unit:ident: $base:ty) -> Self;
        }
    } => {
        #[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

        impl const std::ops::Add<$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl const std::ops::Add<$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl const std::ops::Add<&$vector> for $vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl const std::ops::Add<&$vector> for &$vector {
            type Output = $vector;
            fn add(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl const std::ops::Sub<$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl const std::ops::Sub<$vector> for &$vector {
            type Output = $vector;
            fn sub(self, rhs: $vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl const std::ops::Sub<&$vector> for $vector {
            type Output = $vector;
            fn sub(self, rhs: &$vector) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }

        impl const std::ops::Sub<&$vector> for &$vector {
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

        impl const std::ops::Mul<$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl const std::ops::Mul<$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x * rhs,
                    y: self.y * rhs,
                }
            }
        }

        impl const std::ops::Mul<&$base> for $vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl const std::ops::Mul<&$base> for &$vector {
            type Output = $vector;
            fn mul(self, rhs: &$base) -> Self::Output {
                Self::Output {
                    x: self.x * *rhs,
                    y: self.y * *rhs,
                }
            }
        }

        impl const std::ops::Mul<$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<$vector> for &$base {
            type Output = $vector;
            fn mul(self, rhs: $vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$vector> for $base {
            type Output = $vector;
            fn mul(self, rhs: &$vector) -> Self::Output {
                rhs * self
            }
        }

        impl const std::ops::Mul<&$vector> for &$base {
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

        impl const std::ops::Div<$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl const std::ops::Div<$base> for &$vector {
            type Output = $vector;
            fn div(self, rhs: $base) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                }
            }
        }

        impl const std::ops::Div<&$base> for $vector {
            type Output = $vector;
            fn div(self, rhs: &$base) -> Self::Output {
                self / *rhs
            }
        }

        impl const std::ops::Div<&$base> for &$vector {
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

        impl const std::ops::Neg for $vector {
            type Output = $vector;
            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                }
            }
        }

        impl const std::ops::Neg for &$vector {
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
