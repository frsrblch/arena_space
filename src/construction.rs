use crate::*;

#[derive(Debug)]
pub struct Construction {
    pub r#type: Type,
    pub cost: ResourceArray<Mass>,
    pub completion: TimeFloat,
}

dynamic_arena!(Construction);

#[derive(Debug)]
pub enum Type {
    Spaceport(Location),
    Shipyard(Location),
    Mine,
}

#[derive(Debug)]
pub struct ProjectLinks {
    location: Location,
}

#[derive(Debug)]
pub enum Location {
    StarOrbit(Id<Star>),
    BodyOrbit(Id<Body>),
    Surface(Id<Body>),
}

pub struct Constructions {}
