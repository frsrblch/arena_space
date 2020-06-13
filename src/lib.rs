pub use arena_ecs::*;
pub use components::*;

pub mod components;
pub mod star;
pub mod body;

use fnv::{FnvHashMap, FnvHashSet};
pub type HashMap<I, T> = FnvHashMap<I, T>;
pub type HashSet<I> = FnvHashSet<I>;