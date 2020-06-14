use arena_ecs::*;
pub use components::*;
use state::State;

pub mod state;
pub mod components;
pub mod star;
pub mod body;
pub mod colony;

#[allow(dead_code)]
type HashMap<I, T> = fnv::FnvHashMap<I, T>;
#[allow(dead_code)]
type HashSet<I> = fnv::FnvHashSet<I>;