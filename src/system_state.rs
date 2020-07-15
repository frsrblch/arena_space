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
