use super::DurationFloat;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::f64::consts::PI;

pub const DEG: Angle = Angle::in_deg(1.0);

scalar! {
    struct Angle(f64) {
        fn in_rad(radians) -> Self;
    }
}

impl Angle {
    pub const fn in_deg(degrees: f64) -> Self {
        Self::new(degrees * Self::RAD_PER_DEG)
    }

    pub fn sin(self) -> f64 {
        self.value.sin()
    }

    pub fn cos(self) -> f64 {
        self.value.cos()
    }

    const RAD_PER_DEG: f64 = PI / 180.0;

    pub const TWO_PI: Self = Angle::in_rad(2.0 * PI);

    pub const NEG_TWO_PI: Self = Angle::in_rad(-2.0 * PI);
}

impl Distribution<Angle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle::in_rad(rng.gen_range(-PI, PI))
    }
}

scalar! {
    struct AngularSpeed(f64) {
        fn in_rad_per_s(rad_per_s) -> Self;
    }
}

scalar_div!(Angle | DurationFloat = AngularSpeed);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_gen() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let angle: Angle = rng.gen();

            assert!(angle >= Angle::in_rad(-PI));
            assert!(angle < Angle::in_rad(PI))
        }
    }
}
