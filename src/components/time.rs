use super::*;
use chrono::Duration;
use std::cmp::Ordering;

pub const S: DurationFloat = DurationFloat::in_s(1.0);
pub const MIN: DurationFloat = DurationFloat::in_s(60.0);
pub const HR: DurationFloat = DurationFloat::in_hours(1.0);
pub const DAY: DurationFloat = DurationFloat::in_hours(24.0);
pub const YR: DurationFloat = DurationFloat::in_days(365.25);

/// Elapsed game time in seconds. Distinct from Duration, which is a relative amount of time.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TimeFloat {
    value: f64,
}

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

    pub const NEVER: Self = Self {
        value: f64::INFINITY,
    };
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
        self.value.partial_cmp(&other.value).unwrap()
    }
}

scalar! {
    struct DurationFloat(f64) {
        fn in_s(seconds) -> Self;
    }
}

impl DurationFloat {
    pub const fn in_days(days: f64) -> Self {
        Self::in_s(days * Self::SECONDS_PER_DAY)
    }

    pub const fn in_hours(hours: f64) -> Self {
        Self::in_s(hours * Self::SECONDS_PER_HOUR)
    }

    pub fn days(&self) -> Days {
        Days(*self)
    }

    pub const SECONDS_PER_DAY: f64 = Self::SECONDS_PER_HOUR * 24.0;

    pub const SECONDS_PER_HOUR: f64 = 3600.0;

    pub const INFINITY: DurationFloat = DurationFloat::new(f64::INFINITY);
}

impl From<chrono::Duration> for DurationFloat {
    fn from(duration: Duration) -> Self {
        let seconds = duration.num_milliseconds() as f64 / 1e3;
        DurationFloat::in_s(seconds)
    }
}

impl From<DurationFloat> for chrono::Duration {
    fn from(duration: DurationFloat) -> Self {
        let microseconds = (duration.value * 1e6) as i64;
        Duration::microseconds(microseconds)
    }
}

impl From<DurationFloat> for std::time::Duration {
    fn from(duration: DurationFloat) -> Self {
        let microseconds = (duration.value * 1e6) as u64;
        std::time::Duration::from_micros(microseconds)
    }
}

pub struct Days(DurationFloat);

impl Display for Days {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let days = self.0 / DurationFloat::in_days(1.0);
        write!(f, "{:.1} days", days)
    }
}

#[test]
fn duration_float_from_duration() {
    let one_second = chrono::Duration::seconds(1);
    let one_second = DurationFloat::from(one_second);

    assert_eq!(DurationFloat::in_s(1.0), one_second);
}
