use crate::body::Bodies;
use crate::colony::Colonies;
// use crate::nation::Nations;
use crate::ships::Freighters;
use crate::star::Stars;
use crate::time::{DateTime, TimeState};

#[derive(Debug, Default)]
pub struct State {
    pub time: TimeState,
    pub star: Stars,
    pub body: Bodies,
    // pub nation: Nations,
    pub colony: Colonies,
    pub freighter: Freighters,
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
        self.colony.print();
        println!();
    }
}
