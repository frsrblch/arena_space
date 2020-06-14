use crate::*;
use crate::star::Star;

pub mod planet {
    use super::*;

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
}


#[derive(Debug, Default)]
pub struct Body {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub mass: Component<Self, Mass>,
    pub radius: Component<Self, Length>,
    pub orbit: Component<Self, Orbit>,
    pub surface: Component<Self, Surface>,

    pub star: Component<Self, Id<Star>>,
}

fixed_arena!(Body, u32);

impl Body {
    pub fn create(&mut self, row: BodyRow, links: BodyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.mass.insert(id, row.mass);
        self.radius.insert(id, row.radius);
        self.surface.insert(id, row.surface);
        self.orbit.insert(id, self.get_orbit(links.parent, row.orbit));

        self.star.insert(id, links.star);

        id
    }

    fn get_orbit(&self, parent: Option<Id<Body>>, params: OrbitParams) -> Orbit {
        let parent = parent
            .and_then(|parent| self.orbit.get(parent))
            .map(|orbit| {
                assert!(orbit.parent.is_none(), "Cannot use a moon as a parent body.");
                orbit.params
            });

        Orbit {
            params,
            parent,
        }
    }

    pub fn get_position(&self, id: Id<Body>, time: Time) -> Option<Position> {
        self.orbit.get(id)?
            .calculate_position(time)
            .into()
    }

    pub fn get_distance(&self, from: Id<Body>, to: Id<Body>, time: Time) -> Option<Distance> {
        let system_from = self.star.get(from)?;
        let system_to = self.star.get(to)?;

        if system_from == system_to {
            let from = self.get_position(from, time)?;
            let to = self.get_position(to, time)?;

            Some(to - from)
        } else {
            unimplemented!("Distance between bodies in different systems not implemented.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: OrbitParams,
    pub surface: Surface,
}

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub star: Id<Star>,
    pub parent: Option<Id<Body>>,
}