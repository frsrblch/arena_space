use super::*;
use std::cmp::Ordering;

/// Elapsed game time in seconds. Distinct from Duration, which is a relative amount of time.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TimeFloat { pub value: f64 }

impl TimeFloat {
    pub fn in_s(seconds: f64) -> Self {
        Self::new(seconds)
    }

    pub fn in_days(days: f64) -> Self {
        Self::in_s(days * DurationFloat::SECONDS_PER_DAY)
    }

    fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Div for TimeFloat {
    type Output = f64;
    fn div(self, rhs: Self) -> f64 {
        self.value / rhs.value
    }
}

impl Add<DurationFloat> for TimeFloat {
    type Output = Self;
    fn add(self, rhs: DurationFloat) -> Self {
        Self::new(self.value + rhs.value)
    }
}

impl AddAssign<DurationFloat> for TimeFloat {
    fn add_assign(&mut self, rhs: DurationFloat) {
        self.value += rhs.value;
    }
}

impl Sub<DurationFloat> for TimeFloat {
    type Output = Self;
    fn sub(self, rhs: DurationFloat) -> Self {
        Self::new(self.value - rhs.value)
    }
}

impl Sub for TimeFloat {
    type Output = DurationFloat;
    fn sub(self, rhs: Self) -> DurationFloat {
        DurationFloat::in_s(self.value - rhs.value)
    }
}

impl Div<DurationFloat> for TimeFloat {
    type Output = f64;
    fn div(self, rhs: DurationFloat) -> Self::Output {
        self.value / rhs.value
    }
}

impl Eq for TimeFloat {}

impl PartialOrd for TimeFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for TimeFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value
            .partial_cmp(&other.value)
            .unwrap()
    }
}