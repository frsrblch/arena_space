use crate::components::DurationFloat;
use crate::state::State;
use crate::time::DateTime;
use chrono::Duration;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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

array_enum!(
    System {
        ColonyProductionCycle,
        ColonyFoodProduction, // remove
        NationFoodTargets,
        ColonyFoodProductionRate,
        ColonyPopulation,
        ColonyFoodDecay,
        PrintState,
    }
);

impl System {
    fn run(self, state: &mut State) {
        match self {
            System::ColonyProductionCycle => state.colony.economy.production_cycle(&state.colony.alloc),
            System::NationFoodTargets => state.nation.update_food_targets(&state.colony),
            System::ColonyFoodProductionRate => {
                state.colony.update_food_production_rate(&state.nation, &state.body)
            },
            System::ColonyFoodProduction => state.colony.produce_and_consume_food(),
            System::ColonyPopulation => state.colony.update_population(&mut state.body),
            System::ColonyFoodDecay => state.colony.food_decay(),
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
            System::ColonyFoodProductionRate => DurationFloat::in_days(5.0),
            System::ColonyFoodProduction => DurationFloat::in_days(1.0),
            System::ColonyPopulation => DurationFloat::in_days(5.0),
            System::ColonyFoodDecay => DurationFloat::in_days(30.0),
            System::PrintState => DurationFloat::in_days(90.0),
        }
    }

    pub const fn get_interval_as_year_fraction(self) -> f64 {
        self.get_interval_float().value / DurationFloat::in_days(365.25).value
    }
}

#[derive(Debug)]
pub struct Systems {
    pub queue: BinaryHeap<Reverse<UpdateToken>>,
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
            .map(Reverse)
            .collect();

        Self { queue }
    }

    pub fn update(&mut self, state: &mut State, target: DateTime) {
        while !self.target_reached(target) {
            let current_update = self.pop().expect("system queue should never be empty");
            let next_update = current_update.run(state);
            self.push(next_update);
        }

        state.time.set_date_time(target);
    }

    fn target_reached(&self, target: DateTime) -> bool {
        self.peek()
            .map(|token| token.next_update > target)
            .unwrap_or(true)
    }

    fn peek(&self) -> Option<&UpdateToken> {
        self.queue.peek().map(|rev| &rev.0)
    }

    fn pop(&mut self) -> Option<UpdateToken> {
        self.queue.pop().map(|rev| rev.0)
    }

    fn push(&mut self, token: UpdateToken) {
        self.queue.push(Reverse(token));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_token_next_update_ord() {
        let next_update = crate::time::starting_date();
        let system = System::NationFoodTargets;

        let first = UpdateToken {
            next_update,
            system,
        };

        let second = UpdateToken {
            next_update: first.next_update + first.system.get_interval(),
            system,
        };

        assert!(first < second);
    }

    #[test]
    fn update_token_system_ord() {
        let next_update = crate::time::starting_date();

        let first = UpdateToken {
            next_update,
            system: System::NationFoodTargets,
        };

        let second = UpdateToken {
            next_update,
            system: System::ColonyFoodProductionRate,
        };

        assert!(first < second);
    }

    #[test]
    fn system_ord() {
        let next_update = crate::time::starting_date();
        assert!(
            System::ColonyFoodProduction.get_first_token(next_update)
                < System::ColonyFoodProductionRate.get_first_token(next_update)
        );
        assert!(
            System::NationFoodTargets.get_first_token(next_update)
                < System::ColonyFoodProductionRate.get_first_token(next_update)
        );
    }
}
