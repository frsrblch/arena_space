use crate::star::Star;
use crate::body::Body;
use crate::colony::Colony;
use crate::nation::Nation;
use crate::components::Time;

#[derive(Debug, Default)]
pub struct State {
    pub time: Time,
    pub star: Star,
    pub body: Body,
    pub colony: Colony,
    pub nation: Nation,
}

impl State {
    pub fn set_time(&mut self, time: Time) {
        self.time = time;
    }

    pub fn update(&mut self) {
        self.colony.update_food(self.time);
        self.nation.update_agri_production(&self.colony, self.time);
    }
}