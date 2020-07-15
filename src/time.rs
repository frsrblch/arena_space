use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration};
use crate::components::TimeFloat;
use std::ops::AddAssign;

pub type DateTime = NaiveDateTime;
type StdDuration = std::time::Duration;

#[derive(Debug)]
pub struct TimeState {
    game_time: DateTime,
    time_float: TimeFloat,
    start_date: DateTime,
}

impl TimeState {
    pub fn new(start_date: DateTime) -> Self {
        TimeState {
            game_time: start_date,
            time_float: TimeFloat::in_s(0.0),
            start_date,
        }
    }

    pub fn set_date_time(&mut self, date_time: DateTime) {
        debug_assert!(date_time >= self.game_time);
        self.game_time = date_time;
        self.time_float = Self::get_as_float(self.game_time)
    }

    pub fn get_time(&self) -> DateTime {
        self.game_time
    }

    pub fn get_time_float(&self) -> TimeFloat {
        self.time_float
    }

    fn get_as_float(date_time: DateTime) -> TimeFloat {
        let game_duration: Duration = date_time - starting_date();
        let seconds = game_duration.num_milliseconds() as f64 / 1e3;
        TimeFloat::in_s(seconds)
    }
}

impl AddAssign<StdDuration> for TimeState {
    fn add_assign(&mut self, rhs: StdDuration) {
        let new_date_time = self.game_time + Duration::from_std(rhs).expect("invalid DateTime after adding Duration");
        self.set_date_time(new_date_time);
    }
}

impl Default for TimeState {
    fn default() -> Self {
        Self::new(starting_date())
    }
}

pub fn starting_date() -> DateTime {
    let date = NaiveDate::from_ymd(2050, 1, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    NaiveDateTime::new(date, time)
}

#[test]
fn time_size_tests() {
    assert_eq!(16, std::mem::size_of::<Duration>());
    assert_eq!(12, std::mem::size_of::<chrono::DateTime<chrono::Utc>>());
}
