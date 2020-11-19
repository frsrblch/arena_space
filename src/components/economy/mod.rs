use gen_id::{Component, IdMap, ValidId};
use iter_context::ContextualIterator;
use std::fmt::{Display, Formatter};

use Facility::*;
use Resource::*;

pub use credits::*;
pub use facility::*;
pub use resource::*;

mod credits;
mod facility;
mod resource;

#[cfg(test)]
mod test;
