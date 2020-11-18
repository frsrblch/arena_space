use crate::components::{Mass, Resource};

#[derive(Debug)]
pub struct CargoEntry {
    pub resource: Resource,
    pub amount: Mass,
}
