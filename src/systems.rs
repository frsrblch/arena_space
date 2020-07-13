use std::collections::BTreeSet;
use crate::time::DateTime;
use chrono::Duration;
use crate::state::State;

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum System {
    ColonyFoodProduction,
    NationFoodTargets,
    ColonyFoodProductionRate,
}

impl System {
    fn iter() -> impl Iterator<Item=Self> {
        vec![
            System::ColonyFoodProduction,
            System::NationFoodTargets,
            System::ColonyFoodProductionRate,
        ]
            .into_iter()
    }

    fn run(self, state: &mut State) {
        match self {
            System::NationFoodTargets => state.nation.update_food_targets(&state.colony),
            System::ColonyFoodProductionRate => state.colony.update_food_production_rate(&state.nation, &state.body),
            System::ColonyFoodProduction => state.colony.produce_and_consume_food(),
        }
    }

    fn get_first_token(self) -> UpdateToken {
        UpdateToken {
            next_update: crate::time::starting_date(),
            system: self
        }
    }

    pub fn get_interval(self) -> Duration {
        match self {
            System::NationFoodTargets => Duration::days(30),
            System::ColonyFoodProductionRate => Duration::days(5),
            System::ColonyFoodProduction => Duration::days(1),
        }
    }
}

#[derive(Debug)]
pub struct Systems {
    pub queue: BTreeSet<UpdateToken>,
}

impl Default for Systems {
    fn default() -> Self {
        let queue = System::iter()
            .map(System::get_first_token)
            .collect();

        Self { queue }
    }
}

impl Systems {
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
        self.queue.first()
    }

    fn pop(&mut self) -> Option<UpdateToken> {
        self.queue.pop_first()
    }

    fn push(&mut self, token: UpdateToken) {
        self.queue.insert(token);
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
        assert!(System::ColonyFoodProduction < System::ColonyFoodProductionRate);
        assert!(System::NationFoodTargets < System::ColonyFoodProductionRate);
    }
}