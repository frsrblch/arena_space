#![feature(map_first_last)]

use arena_ecs::*;
use components::*;
use state::State;
use crate::systems::Systems;
use crate::time::DateTime;

pub mod state;
pub mod components;
pub mod systems;
pub mod time;
pub mod star;
pub mod body;
pub mod colony;
pub mod nation;

#[allow(dead_code)]
type HashMap<I, T> = fnv::FnvHashMap<I, T>;
#[allow(dead_code)]
type HashSet<I> = fnv::FnvHashSet<I>;

#[derive(Debug, Default)]
pub struct SystemState {
    pub state: State,
    pub systems: Systems,
}

impl SystemState {
    pub fn update(&mut self, target: DateTime) {
        self.systems.update(&mut self.state, target);
    }
}