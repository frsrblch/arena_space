use super::{DurationFloat, Speed};

scalar! {
    struct Acceleration(f64) {
        fn in_m_per_s2(m_per_s2) -> Self;
    }
}

scalar_div!(Speed | DurationFloat = Acceleration);
