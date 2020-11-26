use crate::components::DurationFloat;
use crate::state::State;
use crate::systems::SystemQueue;
use crate::time::DateTime;

#[derive(Debug, Default)]
pub struct SystemState {
    pub state: State,
    pub systems: SystemQueue,
}

impl SystemState {
    pub fn new(start_date: DateTime) -> Self {
        Self {
            state: State::new(start_date),
            systems: SystemQueue::new(start_date),
        }
    }

    pub fn update(&mut self, target: DateTime) {
        self.systems.update(&mut self.state, target);
    }

    pub fn update_by(&mut self, duration: DurationFloat) {
        self.systems.update_by(&mut self.state, duration);
    }
}
