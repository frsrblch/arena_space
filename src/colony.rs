use crate::*;
use crate::body::Body;
use std::num::NonZeroU32;

#[derive(Debug, Default)]
pub struct Colony {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub population: Component<Self, Population>,

    pub body: Component<Self, Id<Body>>,
}

impl Arena for Colony {
    type Index = u32;
    type Generation = NonZeroU32;
    type Allocator = DynamicAllocator<Self>;
}

impl Colony {
    pub fn create(&mut self, row: ColonyRow, links: ColonyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);

        self.body.insert(id, links.body);

        id.id
    }
}

#[derive(Debug, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: Population,
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
}