use super::{Duration, Speed};
use crate::components::{Length, Mass};
use crate::constants::G;

scalar! {
    struct Acceleration(f64) {
        fn in_m_per_s2(m_per_s2) -> Self;
    }
}

scalar_div!(Speed | Duration = Acceleration);

impl Acceleration {
    pub fn from_gravity(mass: Mass, distance: Length) -> Self {
        let g = G * mass.value / (distance.value * distance.value);
        Acceleration::in_m_per_s2(g)
    }
}
