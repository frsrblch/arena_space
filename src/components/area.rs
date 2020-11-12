use super::Length;

scalar!(Area, square_meters, in_m2);

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
