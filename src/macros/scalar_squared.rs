macro_rules! scalar_squared {
    { $base:ident ^ 2 = $squared:ident } => {
        impl const $crate::Squared for $base {
            type Output = $squared;

            fn squared(self) -> Self::Output {
                $squared::new(self.value() * self.value())
            }
        }

        impl $crate::Sqrt for $squared {
            type Output = $base;

            fn sqrt(self) -> Self::Output {
                $base::new(self.value().sqrt())
            }
        }

        impl const std::ops::Mul<$base> for $base {
            type Output = $squared;

            fn mul(self, rhs: $base) -> Self::Output {
                $squared::new(self.value * rhs.value)
            }
        }

        impl const std::ops::Mul<$base> for &$base {
            type Output = $squared;

            fn mul(self, rhs: $base) -> Self::Output {
                $squared::new(self.value * rhs.value)
            }
        }

        impl<'a> const std::ops::Mul<&'a $base> for $base {
            type Output = $squared;

            fn mul(self, rhs: &$base) -> Self::Output {
                $squared::new(self.value * rhs.value)
            }
        }

        impl<'a> const std::ops::Mul<&'a $base> for &'a $base {
            type Output = $squared;

            fn mul(self, rhs: &$base) -> Self::Output {
                $squared::new(self.value * rhs.value)
            }
        }

        impl const std::ops::Div<$base> for $squared {
            type Output = $base;

            fn div(self, rhs: $base) -> Self::Output {
                $base::new(self.value / rhs.value)
            }
        }

        impl const std::ops::Div<$base> for &$squared {
            type Output = $base;

            fn div(self, rhs: $base) -> Self::Output {
                $base::new(self.value / rhs.value)
            }
        }

        impl<'a> const std::ops::Div<&'a $base> for $squared {
            type Output = $base;

            fn div(self, rhs: &$base) -> Self::Output {
                $base::new(self.value / rhs.value)
            }
        }

        impl<'a> const std::ops::Div<&'a $base> for &'a $squared {
            type Output = $base;

            fn div(self, rhs: &$base) -> Self::Output {
                $base::new(self.value / rhs.value)
            }
        }
    }
}
