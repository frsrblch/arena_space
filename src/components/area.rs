use super::Length;

scalar! {
    struct Area(f64) {
        fn in_m2(square_meters) -> Self;
    }
}

impl Area {
    pub fn in_square_km(value: f64) -> Self {
        Self::in_m2(value * 1e6)
    }
}

impl std::ops::Mul<Length> for Length {
    type Output = Area;
    fn mul(self, rhs: Self) -> Self::Output {
        Area::new(self.value * rhs.value)
    }
}
