use crate::*;
use crate::star::{Stars, Star};

pub use components::*;
use crate::geometry::Sphere;

mod components;

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: OrbitParams,
    pub conditions: BodyProperties,
}

fixed_arena!(Body);

#[derive(Debug, Copy, Clone)]
pub struct BodyLinks {
    pub star: Id<Star>,
    pub parent: Option<Id<Body>>,
}

#[derive(Debug, Default)]
pub struct Bodies {
    pub alloc: Allocator<Body>,

    pub name: Component<Body, String>,
    pub mass: Component<Body, Mass>,
    pub radius: Component<Body, Length>,
    pub orbit: Component<Body, Orbit>,
    pub properties: Component<Body, BodyProperties>,

    pub star: Component<Body, Id<Star>>,
}

impl Bodies {
    pub fn create(&mut self, row: Body, links: BodyLinks) -> Id<Body> {
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

    pub fn get_distance(&self, from: Id<Body>, to: Id<Body>, time: TimeFloat, star: &Stars) -> Distance {
        let system_from = self.star.get(from);
        let system_to = self.star.get(to);

        if system_from == system_to {
            let from = self.get_position(from, time);
            let to = self.get_position(to, time);

            to - from
        } else {
            let from = star.position.get(system_from);
            let to = star.position.get(system_to);

            to - from
        }
    }

    pub fn get_habitability<ID: Indexes<Body>>(&self, id: ID) -> Habitability {
        self.properties.get(id).get_habitability()
    }

    pub fn get_land_area<ID: Indexes<Body> + Copy>(&self, id: ID) -> Area {
        let radius = self.radius.get(id);
        let area = Sphere::with_radius(*radius).get_area();

        match self.properties.get(id).surface {
            Surface::Gaseous => Area::zero(),
            Surface::Volcanic => Area::zero(),
            Surface::Frozen => area,
            Surface::Barren => area,
            Surface::Continental { land } => land.value() * area,
            Surface::Oceanic => Area::zero(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: Body,
    pub moons: Vec<Body>,
}

impl Bodies {
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

pub mod examples {
    use super::*;

    pub fn earth() -> Body {
        Body {
            name: "Earth".to_string(),
            mass: Mass::in_kg(5.972e24),
            radius: Length::in_m(6371e3),
            orbit: OrbitParams {
                radius: Length::in_m(149.60e9),
                period: DurationFloat::in_days(365.25),
                offset: Default::default()
            },
            conditions: BodyProperties {
                surface: Surface::Continental { land: Fraction::new(0.204) },
                pressure: Pressure::Ideal,
                oxygen: AtmosphericOxygen::Ideal,
                hydrosphere: Hydrosphere::Dynamic,
                biosphere: Biosphere::Advanced,
                magnetosphere: Magnetosphere::Present
            }
        }
    }

    pub fn luna() -> Body {
        Body {
            name: "Luna".to_string(),
            mass: Mass::in_kg(7.34767309e22),
            radius: Length::in_m(1737.1e3),
            orbit: OrbitParams {
                radius: Length::in_m(384_400e3),
                period: DurationFloat::in_days(27.322),
                offset: Default::default()
            },
            conditions: BodyProperties {
                surface: Surface::Barren,
                pressure: Pressure::Vacuum,
                oxygen: AtmosphericOxygen::None,
                hydrosphere: Hydrosphere::None,
                biosphere: Biosphere::None,
                magnetosphere: Magnetosphere::Absent
            },
        }
    }
}