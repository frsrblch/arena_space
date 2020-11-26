use crate::colony::Colony;
use gen_id::*;

#[derive(Debug, Copy, Clone)]
pub enum Assignment {
    Route(Id<Colony>, Id<Colony>),
}
