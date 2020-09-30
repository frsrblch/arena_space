use crate::body::Bodies;
use crate::colony::Colonies;
use crate::nation::Nations;
use crate::star::Stars;
use crate::systems::Systems;
use crate::time::{DateTime, TimeState};

#[derive(Debug, Default)]
pub struct State {
    pub time: TimeState,
    pub systems: Systems,
    pub star: Stars,
    pub body: Bodies,
    pub nation: Nations,
    pub colony: Colonies,
}

impl State {
    pub fn new(start_date: DateTime) -> Self {
        Self {
            time: TimeState::new(start_date),
            ..Default::default()
        }
    }

    pub fn print(&self) {
        self.time.print();
    }
}

impl TimeState {
    fn print(&self) {
        println!("{}\n", self.get_time().format("%Y-%m-%d"));
    }
}