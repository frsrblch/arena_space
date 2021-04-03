use crate::components::{Distance, Length};
use std::ops::Mul;

scalar! {
    struct PixelScale(f32) {
        fn in_px_per_m(pixel_per_meter) -> Self;
    }
}

impl Mul<PixelScale> for Distance {
    type Output = (f32, f32);
    #[inline]
    fn mul(self, rhs: PixelScale) -> Self::Output {
        (
            self.x.value as f32 * rhs.value,
            self.y.value as f32 * rhs.value,
        )
    }
}

impl Mul<Distance> for PixelScale {
    type Output = (f32, f32);
    #[inline]
    fn mul(self, rhs: Distance) -> Self::Output {
        rhs * self
    }
}

impl Mul<PixelScale> for Length {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: PixelScale) -> Self::Output {
        self.value as f32 * rhs.value
    }
}

impl Mul<Length> for PixelScale {
    type Output = f32;
    #[inline]
    fn mul(self, rhs: Length) -> Self::Output {
        rhs * self
    }
}
