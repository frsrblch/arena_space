#![allow(incomplete_features)]

#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(map_first_last)]
#![feature(const_float_classify)]

use arena_ecs::*;
use components::*;
use state::State;

#[cfg(test)]
use system_state::SystemState;

pub mod state;
pub mod components;
pub mod systems;
pub mod system_state;
pub mod time;
pub mod star;
pub mod body;
pub mod nation;
pub mod colony;
pub mod spaceport;
pub mod geometry;

#[allow(dead_code)]
type HashMap<I, T> = fnv::FnvHashMap<I, T>;
#[allow(dead_code)]
type HashSet<T> = fnv::FnvHashSet<T>;
