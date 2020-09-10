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

pub mod body;
pub mod colony;
pub mod components;
pub mod geometry;
pub mod nation;
pub mod spaceport;
pub mod star;
pub mod state;
pub mod system_state;
pub mod systems;
pub mod time;

#[allow(dead_code)]
type HashMap<I, T> = fnv::FnvHashMap<I, T>;
#[allow(dead_code)]
type HashSet<T> = fnv::FnvHashSet<T>;
