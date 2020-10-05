use crate::components::DurationFloat;
use crate::state::State;
use crate::time::DateTime;
use chrono::Duration;
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
            next_update: self.next_update + self.system.get_interval(),
            system: self.system,
        }
    }
}

array_enum!(System {
    ColonyProductionCycle,
    NationFoodTargets,
    ColonyPopulation,
    ResourceDecay,
    PrintState,
});

impl System {
    fn run(self, state: &mut State) {
        match self {
            System::ColonyProductionCycle => state.colony.production_cycle(),
            System::NationFoodTargets => state.nation.update_food_targets(&state.colony),
            System::ColonyPopulation => state.colony.update_population(&mut state.body),
            System::ResourceDecay => state.colony.resources.decay(),
            System::PrintState => state.print(),
        }
    }

    fn get_first_token(self, start: DateTime) -> UpdateToken {
        UpdateToken {
            next_update: start,
            system: self,
        }
    }

    pub fn get_interval(self) -> Duration {
        self.get_interval_float().into()
    }

    pub const fn get_interval_float(self) -> DurationFloat {
        match self {
            System::ColonyProductionCycle => DurationFloat::in_days(1.0),
            System::NationFoodTargets => DurationFloat::in_days(30.0),
            System::ColonyPopulation => DurationFloat::in_days(5.0),
            System::ResourceDecay => DurationFloat::in_days(30.0),
            System::PrintState => DurationFloat::in_days(90.0),
        }
    }

    pub const fn get_interval_as_year_fraction(self) -> f64 {
        self.get_interval_float().value / DurationFloat::in_days(365.25).value
    }
}

#[derive(Debug)]
pub struct Systems {
    pub queue: MinHeap<UpdateToken>,
}

impl Default for Systems {
    fn default() -> Self {
        let start_date = crate::time::starting_date();
        Self::new(start_date)
    }
}

impl Systems {
    pub fn new(start_date: DateTime) -> Self {
        let queue = System::array()
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

    fn target_reached(&self, target: DateTime) -> bool {
        let next_update = self.peek().next_update;
        next_update > target
    }

    fn peek(&self) -> &UpdateToken {
        // SAFETY: system queue will never be empty
        self.queue.peek().unwrap()
    }

    fn pop(&mut self) -> UpdateToken {
        // SAFETY: system queue will never be empty
        self.queue.pop().unwrap()
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
        let heap = BinaryHeap::from_iter(iter.into_iter().map(Reverse));
        Self { heap }
    }
}
