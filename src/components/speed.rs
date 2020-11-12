use super::{DurationFloat, Length};

vector_and_scalar!(Velocity, Speed, meters_per_second, in_m_per_s);

scalar_div!(Length, DurationFloat, Speed);

impl Speed {
    pub const C: Speed = Speed::in_m_per_s(299792458.0);
}
