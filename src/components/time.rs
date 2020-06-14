use super::*;

/// Elapsed game time in seconds. Distinct from Duration, which is a relative amount of time.
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Time { pub value: f64 }

impl Time {
    pub fn in_s(seconds: f64) -> Self {
        Self::new(seconds)
    }

    fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Div for Time {
    type Output = f64;
    fn div(self, rhs: Self) -> f64 {
        self.value / rhs.value
    }
}

impl Add<Duration> for Time {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self {
        Self::new(self.value + rhs.value)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        self.value += rhs.value;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;
    fn sub(self, rhs: Duration) -> Self {
        Self::new(self.value - rhs.value)
    }
}

impl Sub for Time {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Duration {
        Duration::in_s(self.value - rhs.value)
    }
}

impl Div<Duration> for Time {
    type Output = f64;
    fn div(self, rhs: Duration) -> Self::Output {
        self.value / rhs.value
    }
}