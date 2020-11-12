#![allow(incomplete_features)]
#![feature(
    const_fn,
    const_generics,
    const_panic,
    const_float_classify,
    const_fn_floating_point_arithmetic,
    iterator_fold_self
)]

use gen_id::*;
use iter_context::ContextualIterator;

use components::*;

pub mod body;
pub mod colony;
#[macro_use]
pub mod components;
pub mod geometry;
pub mod nation;
pub mod ships;
pub mod spaceport;
pub mod star;
pub mod state;
pub mod system_state;
pub mod systems;
pub mod time;

#[allow(dead_code)]
type HashMap<I, T> = fnv::FnvHashMap<I, T>;
