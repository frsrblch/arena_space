use crate::colony::Colony;
use gen_id::*;

#[derive(Debug)]
pub enum Assignment {
    None,
    Route(Id<Colony>, Id<Colony>),
}
