#![allow(incomplete_features)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(const_float_classify)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(iterator_fold_self)]

use components::*;
use gen_id::*;
use state::State;
use typed_iter::*;

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
#[allow(dead_code)]
type HashSet<T> = fnv::FnvHashSet<T>;
