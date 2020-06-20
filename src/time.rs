use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration};
use crate::components::TimeFloat;
use std::ops::AddAssign;

pub type DateTime = chrono::DateTime<chrono::Utc>;
type StdDuration = std::time::Duration;

#[derive(Debug)]
pub struct TimeState {
    game_time: DateTime,
    time_float: TimeFloat,
}

impl TimeState {
    pub fn get_time_float(&self) -> TimeFloat {
        self.time_float
    }

    fn get_as_float(date_time: DateTime) -> TimeFloat {
        let game_duration: chrono::Duration = date_time - starting_date();
        let seconds = game_duration.num_milliseconds() as f64 / 1e3;
        TimeFloat::in_s(seconds)
    }
}

impl AddAssign<StdDuration> for TimeState {
    fn add_assign(&mut self, rhs: StdDuration) {
        self.game_time = self.game_time + Duration::from_std(rhs).unwrap();
        self.time_float = Self::get_as_float(self.game_time)
    }
}

impl Default for TimeState {
    fn default() -> Self {
        TimeState {
            game_time: starting_date(),
            time_float: TimeFloat::in_s(0.0),
        }
    }
}

fn starting_date() -> DateTime {
    let date = NaiveDate::from_ymd(2050, 1, 1);
    let time = NaiveTime::from_hms(0, 0, 0);
    let date_time = NaiveDateTime::new(date, time);
    DateTime::from_utc(date_time, chrono::Utc)
}

#[test]
fn time_size_tests() {
    assert_eq!(16, std::mem::size_of::<Duration>());
    assert_eq!(12, std::mem::size_of::<chrono::DateTime<chrono::Utc>>());

}
