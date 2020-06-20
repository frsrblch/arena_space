use crate::*;
use crate::star::Star;

pub use components::*;
mod components;

#[derive(Debug, Default)]
pub struct Body {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub mass: Component<Self, Mass>,
    pub radius: Component<Self, Length>,
    pub orbit: Component<Self, Orbit>,
    pub properties: Component<Self, BodyProperties>,

    pub star: Component<Self, Id<Star>>,
}

fixed_arena!(Body, u32);

impl Body {
    pub fn create(&mut self, row: BodyRow, links: BodyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.mass.insert(id, row.mass);
        self.radius.insert(id, row.radius);
        self.orbit.insert(id, self.get_orbit(links.parent, row.orbit));
        self.properties.insert(id, row.conditions);

        self.star.insert(id, links.star);

        id
    }

    fn get_orbit(&self, parent: Option<Id<Body>>, params: OrbitParams) -> Orbit {
        let parent = parent
            .map(|parent| self.orbit.get(parent))
            .map(|orbit| {
                assert!(orbit.parent.is_none(), "Cannot use a moon as a parent body.");
                orbit.params
            });

        Orbit {
            params,
            parent,
        }
    }

    pub fn get_position(&self, id: Id<Body>, time: TimeFloat) -> Position {
        self.orbit.get(id)
            .calculate_position(time)
    }

    pub fn get_distance(&self, from: Id<Body>, to: Id<Body>, time: TimeFloat, _star: &Star) -> Distance {
        let system_from = self.star.get(from);
        let system_to = self.star.get(to);

        if system_from == system_to {
            let from = self.get_position(from, time);
            let to = self.get_position(to, time);

            to - from
        } else {
            unimplemented!("Distance between bodies in different systems not implemented.")
        }
    }

    pub fn get_habitability(&self, id: Id<Body>) -> Habitability {
        self.properties.get(id).get_habitability()
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: OrbitParams,
    pub conditions: BodyProperties,
}

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub star: Id<Star>,
    pub parent: Option<Id<Body>>,
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: BodyRow,
    pub moons: Vec<BodyRow>,
}

impl Body {
    pub fn create_planet(&mut self, row: Planet, star: Id<Star>) -> Id<Body> {
        let links = BodyLinks {
            star,
            parent: None,
        };

        let planet = self.create(row.body, links);

        let moon_links = BodyLinks {
            star: links.star,
            parent: Some(planet),
        };

        for moon in row.moons {
            self.create(moon, moon_links);
        }

        planet
    }
}
