pub use arena_ecs::*;
pub use components::*;

pub mod components;
pub mod star;
pub mod body;
pub mod colony;

pub type HashMap<I, T> = fnv::FnvHashMap<I, T>;
pub type HashSet<I> = fnv::FnvHashSet<I>;

#[derive(Debug, Default)]
pub struct State {
    pub star: star::Star,
    pub body: body::Body,
    pub colony: colony::Colony,
}