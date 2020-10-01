use crate::state::State;
use crate::systems::Systems;
use crate::time::DateTime;

#[derive(Debug, Default)]
pub struct SystemState {
    pub state: State,
    pub systems: Systems,
}

impl SystemState {
    pub fn new(start_date: DateTime) -> Self {
        Self {
            state: State::new(start_date),
            systems: Systems::new(start_date),
        }
    }

    pub fn update(&mut self, target: DateTime) {
        self.systems.update(&mut self.state, target);
    }
}

// #[test]
// fn time_trial() {
//     let start = crate::time::get_date(2050, 1, 1);
//     let end = crate::time::get_date(2060, 1, 1);
//     let mut game = SystemState::new(start);
//
//     // TODO add small game state
//
//     let start = std::time::Instant::now();
//     game.update(end);
//     let end = std::time::Instant::now();
//
//     println!("10 years in: {} us", (end - start).as_micros());
//     panic!();
// }