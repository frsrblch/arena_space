use crate::*;
use crate::star::Star;

#[derive(Debug, Default)]
pub struct Body {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub mass: Component<Self, Mass>,
    pub radius: Component<Self, Length>,

    pub star: Component<Self, Id<Star>>,

    pub planets: HashMap<Id<Self>, Orbit>,
    pub moons: HashMap<Id<Self>, MoonOrbit>,
}

impl Arena for Body {
    type Index = u32;
    type Generation = ();
    type Allocator = FixedAllocator<Self>;
}

impl Body {
    pub fn create(&mut self, row: BodyRow, link: BodyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.mass.insert(id, row.mass);
        self.radius.insert(id, row.radius);

        self.star.insert(id, link.star);

        if let Some(parent) = link.parent {
            self.moons.insert(id, MoonOrbit {
                parent,
                orbit: row.orbit,
            });
        } else {
            self.planets.insert(id, row.orbit);
        }

        id
    }
}

#[derive(Debug)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: Orbit,
}

#[derive(Debug)]
pub struct BodyLinks {
    star: Id<Star>,
    parent: Option<Id<Body>>,
}

#[derive(Debug)]
pub struct Orbit {
    pub radius: Length,
    pub period: Time,
    pub offset: Angle,
}

#[derive(Debug)]
pub struct MoonOrbit {
    pub parent: Id<Body>,
    pub orbit: Orbit,
}