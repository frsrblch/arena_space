use super::{Duration, Length, M, S};

vector_and_scalar! {
    struct Velocity([struct Speed(f64); 2]) {
        fn in_m_per_s(meters_per_second) -> Self;
    }
}

scalar_div! { Length | Duration = Speed }

impl Speed {
    pub const C: Speed = 299792458.0 * M / S;
}
