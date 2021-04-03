#![allow(incomplete_features)]
#![feature(
    const_fn,
    const_generics,
    const_option,
    const_panic,
    const_float_classify,
    const_trait_impl,
    const_trait_bound_opt_out,
    const_fn_floating_point_arithmetic,
    bool_to_option
)]

pub use gen_id::*;
pub use iter_context::ContextualIterator;

use components::*;

#[macro_use]
pub mod macros;

pub mod action;
pub mod body;
pub mod colony;
pub mod components;
pub mod construction;
pub mod entity;
pub mod ftl;
pub mod geometry;
pub mod resources;
pub mod ships;
pub mod spaceport;
pub mod star;
pub mod state;
pub mod system_state;
pub mod systems;
pub mod time;

pub use body::{Bodies, Body};
pub use colony::{Colonies, Colony};
pub use star::{Star, Stars};
pub use state::State;

#[allow(dead_code)]
type HashMap<K, V> = fnv::FnvHashMap<K, V>;

#[allow(dead_code)]
type HashSet<K> = fnv::FnvHashSet<K>;
