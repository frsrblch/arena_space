use crate::components::{Duration, TimeFloat};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

pub type DateTime = NaiveDateTime;
pub type StdDuration = std::time::Duration;
pub type ChronoDuration = chrono::Duration;

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
        self.time_float = self.calculate_time_float();
    }

    pub fn get_date_time(&self) -> DateTime {
        self.game_time
    }

    pub fn get_time(&self) -> TimeFloat {
        self.time_float
    }

    fn calculate_time_float(&self) -> TimeFloat {
        let duration: ChronoDuration = self.game_time - self.start_date;
        let seconds = duration.num_milliseconds() as f64 / 1e3;
        TimeFloat::in_s(seconds)
    }

    pub fn print(&self) {
        println!("{}\n", self);
    }
}

impl Display for TimeState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.game_time.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl AddAssign<StdDuration> for TimeState {
    fn add_assign(&mut self, rhs: StdDuration) {
        let duration = ChronoDuration::from_std(rhs).unwrap();
        let new_date_time = self.game_time + duration;
        self.set_date_time(new_date_time);
    }
}

impl AddAssign<Duration> for TimeState {
    fn add_assign(&mut self, rhs: Duration) {
        self.add_assign(StdDuration::from(rhs));
    }
}

impl Default for TimeState {
    fn default() -> Self {
        Self::new(starting_date())
    }
}

fn starting_date() -> DateTime {
    get_date(2050, 1, 1)
}

pub fn get_date(year: i32, month: u32, day: u32) -> DateTime {
    let date = NaiveDate::from_ymd(year, month, day);
    let time = NaiveTime::from_hms(0, 0, 0);
    NaiveDateTime::new(date, time)
}

#[test]
fn time_size_tests() {
    assert_eq!(16, std::mem::size_of::<ChronoDuration>());
    assert_eq!(12, std::mem::size_of::<chrono::DateTime<chrono::Utc>>());
}
