use crate::star::Stars;
use crate::body::Bodies;
use crate::colony::Colony;
use crate::nation::Nation;
use crate::time::TimeState;

#[derive(Debug, Default)]
pub struct State {
    pub time: TimeState,
    pub star: Stars,
    pub body: Bodies,
    pub nation: Nation,
    pub colony: Colony,
}

impl State {
    pub fn update(&mut self, mut interval: std::time::Duration) {
        while interval > ONE_DAY {
            self.time += ONE_DAY;
            interval -= ONE_DAY;

            self.update_daily();
        }

        self.time += interval;
        self.update_all();
    }

    fn update_daily(&mut self) {
        let time = self.time.get_time_float();

        self.colony.produce_and_consume_food(time);
        self.nation.update_agri_production(&self.colony, time);
        self.colony.update_production(&self.nation, &self.body, time)
    }

    fn update_all(&mut self) {
        self.update_daily();
    }
}

const ONE_DAY: std::time::Duration = std::time::Duration::from_secs(24 * 3600);
