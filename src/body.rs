use crate::*;
use habitability::Habitability;
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

pub use components::*;
mod components {
    #[derive(Debug, Copy, Clone)]
    pub struct Conditions {
        surface: Surface,
        pressure: Pressure,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Surface {
        Barren,
        Gaseous,
        Continental,
        Volcanic,
        Oceanic,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Pressure {
        Vacuum,
        Thin,
        Normal,
        High,
        Crushing,
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
    pub atmosphere: Component<Self, Pressure>,

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
        self.surface.insert(id, row.surface);
        self.atmosphere.insert(id, row.atmosphere);

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

    pub fn get_habitability(&self, id: Id<Body>) -> Option<Habitability> {
        let surface = *self.surface.get(id)?;
        let atmosphere = *self.atmosphere.get(id)?;

        Habitability::new(surface, atmosphere).into()
    }
}

#[derive(Debug, Clone)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: OrbitParams,
    pub surface: Surface,
    pub atmosphere: Pressure,
}

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub star: Id<Star>,
    pub parent: Option<Id<Body>>,
}

mod habitability {
    use super::*;
    use Surface::*;
    use Pressure::*;
    use Habitability::*;

    /// The ability of an environment to support human life.
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Habitability {
        /// Unable to support human life in any capacity, (i.e., Jupiter, Venus).
        Uninhabitable,
        /// Unable to support human life with significant assistance (i.e., Mars, full vacuum).
        Hostile,
        /// Barely able to support human life (i.e., dessert, tundra, high altitudes).
        Marginal,
        /// Able to support human life (i.e., grasslands, forests).
        Optimal,
    }

    impl Default for Habitability {
        fn default() -> Self {
            Habitability::Uninhabitable
        }
    }

    impl Habitability {
        pub fn new(surface: Surface, pressure: Pressure) -> Self {
            Self::for_surface(surface).min(Self::for_pressure(pressure))
        }

        fn for_surface(surface: Surface) -> Self {
            match surface {
                Barren => Marginal,
                Gaseous => Uninhabitable,
                Continental => Optimal,
                Volcanic => Hostile,
                Oceanic => Marginal,
            }
        }

        fn for_pressure(pressure: Pressure) -> Self {
            match pressure {
                Vacuum => Hostile,
                Thin => Marginal,
                Normal => Optimal,
                High => Marginal,
                Crushing => Uninhabitable,
            }
        }
    }
}