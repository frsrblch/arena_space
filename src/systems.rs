use crate::components::*;
use crate::state::State;
use crate::time::{DateTime, TimeState};
use chrono::Duration as ChronoDuration;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UpdateToken {
    pub next_update: DateTime,
    pub system: System,
}

impl UpdateToken {
    fn run(self, state: &mut State) -> UpdateToken {
        state.time.set_date_time(self.next_update);

        self.system.run(state);

        UpdateToken {
            next_update: self.next_update + self.system.get_chrono_interval(),
            system: self.system,
        }
    }
}

array_enum! {
    enum System {
        FreighterState,
        ColonyProductionCycle,
        ColonyPopulation,
        ResourceDecay,
        PrintState,
        ShippingAverage,
    }
}

impl System {
    fn run(self, state: &mut State) {
        match self {
            System::FreighterState => {
                state
                    .freighter
                    .update(&state.time, &mut state.colony, &state.body, &state.star)
            }
            System::ColonyProductionCycle => state.colony.production_cycle(),
            System::ColonyPopulation => state.colony.update_population(&mut state.body),
            System::ResourceDecay => state.colony.resources.decay(),
            System::PrintState => {} // state.print(),
            System::ShippingAverage => state.colony.resources.update_shipping_avg(),
        }
    }

    fn get_first_token(self, start: DateTime) -> UpdateToken {
        UpdateToken {
            next_update: start,
            system: self,
        }
    }

    pub fn get_chrono_interval(self) -> ChronoDuration {
        self.get_interval().into()
    }

    pub const fn get_interval(self) -> Duration {
        match self {
            System::FreighterState => 10.0 * MIN,
            System::ColonyProductionCycle => 1.0 * DAY,
            System::ColonyPopulation => 5.0 * DAY,
            System::ResourceDecay => 30.0 * DAY,
            System::PrintState => 90.0 * DAY,
            System::ShippingAverage => 7.0 * DAY,
        }
    }

    pub const fn get_interval_as_year_fraction(self) -> f64 {
        self.get_interval() / Duration::in_days(365.25)
    }
}

#[derive(Debug)]
pub struct SystemQueue {
    pub queue: MinHeap<UpdateToken>,
}

impl Default for SystemQueue {
    fn default() -> Self {
        let start = TimeState::default();
        Self::new(start.get_date_time())
    }
}

impl SystemQueue {
    pub fn new(start_date: DateTime) -> Self {
        let queue = System::ARRAY
            .iter()
            .map(|system| system.get_first_token(start_date))
            .collect();

        Self { queue }
    }

    pub fn update(&mut self, state: &mut State, target: DateTime) {
        while !self.target_reached(target) {
            let current_update = self.pop();
            let next_update = current_update.run(state);
            self.push(next_update);
        }

        state.time.set_date_time(target);
    }

    pub fn update_by(&mut self, state: &mut State, duration: Duration) {
        let time = state.time.get_date_time() + ChronoDuration::from(duration);
        self.update(state, time);
    }

    fn target_reached(&self, target: DateTime) -> bool {
        let next_update = self.peek().next_update;
        next_update > target
    }

    fn peek(&self) -> &UpdateToken {
        self.queue.peek().expect("system queue is empty")
    }

    fn pop(&mut self) -> UpdateToken {
        self.queue.pop().expect("system queue is empty")
    }

    fn push(&mut self, token: UpdateToken) {
        self.queue.push(token);
    }
}

#[derive(Debug, Default)]
pub struct MinHeap<T: Ord> {
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MinHeap<T> {
    pub fn push(&mut self, value: T) {
        self.heap.push(Reverse(value));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|rev| rev.0)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|rev| &rev.0)
    }
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let heap = iter.into_iter().map(Reverse).collect();
        Self { heap }
    }
}
