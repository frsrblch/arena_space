pub use arena_ecs::*;
pub use components::*;

pub mod components;
pub mod star;
pub mod body;
pub mod colony;

use fnv::{FnvHashMap, FnvHashSet};
pub type HashMap<I, T> = FnvHashMap<I, T>;
pub type HashSet<I> = FnvHashSet<I>;

#[derive(Debug, Default)]
pub struct State {
    pub star: star::Star,
    pub body: body::Body,
    pub colony: colony::Colony,
}