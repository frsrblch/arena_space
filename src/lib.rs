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
    iterator_fold_self,
    bool_to_option
)]

use gen_id::*;
use iter_context::ContextualIterator;

use components::*;

#[macro_use]
pub mod macros;

pub mod body;
pub mod colony;
pub mod components;
pub mod geometry;
// pub mod nation;
pub mod ftl;
pub mod ships;
pub mod spaceport;
pub mod star;
pub mod state;
pub mod system_state;
pub mod systems;
pub mod time;

#[allow(dead_code)]
type HashMap<K, V> = fnv::FnvHashMap<K, V>;
