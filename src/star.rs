use crate::*;

#[derive(Debug, Default)]
pub struct Star {
    pub alloc: Allocator<Self>,
    pub name: Component<Self, String>,
    pub mass: Component<Self, Mass>,
    pub radius: Component<Self, Length>,
    pub temperature: Component<Self, Temperature>,
}

impl Arena for Star {
    type Index = u16;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Star {
    pub fn create(&mut self, row: StarRow) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.mass.insert(id, row.mass);
        self.radius.insert(id, row.radius);
        self.temperature.insert(id, row.temperature);

        id
    }
}

pub struct StarRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub temperature: Temperature,
}
