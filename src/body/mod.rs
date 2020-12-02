use crate::star::{Star, Stars};
use crate::*;

use crate::geometry::Sphere;
pub use components::*;

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

    pub population: HashMap<Id<Body>, Population>,

    pub star: Component<Body, Id<Star>>,
}

impl Bodies {
    pub fn create(&mut self, row: Body, links: BodyLinks) -> Id<Body> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.mass.insert(id, row.mass);

        self.radius.insert(id, row.radius);

        let orbit = self.get_orbit(links.parent, row.orbit);
        self.orbit.insert(id, orbit);

        self.properties.insert(id, row.conditions);

        self.star.insert(id, links.star);

        id
    }

    fn get_orbit(&self, parent: Option<Id<Body>>, params: OrbitParams) -> Orbit {
        let parent = parent.map(|parent| self.orbit.get(parent)).map(|orbit| {
            assert!(
                orbit.parent.is_none(),
                "Cannot use a moon as a parent body."
            );
            orbit.params
        });

        Orbit { params, parent }
    }

    pub fn get_position<I: ValidId<Body>>(&self, id: I, time: TimeFloat) -> Position {
        self.orbit.get(id).calculate_position(time)
    }

    pub fn get_distance(
        &self,
        from: Id<Body>,
        to: Id<Body>,
        time: TimeFloat,
        star: &Stars,
    ) -> Distance {
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

    pub fn get_habitability(&self, id: Id<Body>) -> Habitability {
        self.properties.get(id).get_habitability()
    }

    pub fn get_land_area<I: ValidId<Body>>(&self, id: I) -> Area {
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

    pub fn get_by_name(&self, name: &str) -> Option<Id<Body>> {
        self.name
            .iter()
            .zip(self.alloc.ids())
            .into_iter()
            .filter_map(|(n, id)| if *n == name { Some(id) } else { None })
            .next()
    }
}

#[derive(Debug, Clone)]
pub struct Planet {
    pub body: Body,
    pub moons: Vec<Body>,
}

impl Bodies {
    pub fn create_planet(&mut self, row: Planet, star: Id<Star>) -> Id<Body> {
        let links = BodyLinks { star, parent: None };

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

pub mod population {
    use super::*;
    use crate::colony::Colonies;
    use std::collections::hash_map::Entry;

    impl Bodies {
        pub fn sum_population(&mut self, colonies: &Colonies) {
            self.population.clear();
            self.add_colony_population(colonies);
        }

        fn add_colony_population(&mut self, colonies: &Colonies) {
            let population = colonies.people.population.iter();
            let body = colonies.body.iter();

            for (pop, body) in population.zip(body) {
                self.add_population(body, pop);
            }
        }

        fn add_population(&mut self, body: &Id<Body>, colony_population: &Population) {
            match self.population.entry(*body) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += colony_population;
                }
                Entry::Vacant(entry) => {
                    entry.insert(*colony_population);
                }
            }
        }
    }
}

pub mod examples {
    use super::*;

    pub fn planet_earth() -> Planet {
        Planet {
            body: earth(),
            moons: vec![luna()],
        }
    }

    pub fn earth() -> Body {
        Body {
            name: "Earth".to_string(),
            mass: Mass::in_kg(5.972e24),
            radius: Length::in_m(6371e3),
            orbit: OrbitParams::from_period(
                Length::in_m(149.60e9),
                Duration::in_days(365.25),
                Default::default(),
            ),
            conditions: BodyProperties {
                surface: Surface::Continental {
                    land: Fraction::new(0.204),
                },
                pressure: Pressure::Ideal,
                oxygen: AtmosphericOxygen::Ideal,
                hydrosphere: Hydrosphere::Dynamic,
                biosphere: Biosphere::Advanced,
                magnetosphere: Magnetosphere::Present,
            },
        }
    }

    pub fn luna() -> Body {
        Body {
            name: "Luna".to_string(),
            mass: Mass::in_kg(7.34767309e22),
            radius: Length::in_m(1737.1e3),
            orbit: OrbitParams::from_period(
                Length::in_m(384_400e3),
                Duration::in_days(27.322),
                Default::default(),
            ),

            conditions: BodyProperties {
                surface: Surface::Barren,
                pressure: Pressure::Vacuum,
                oxygen: AtmosphericOxygen::None,
                hydrosphere: Hydrosphere::None,
                biosphere: Biosphere::None,
                magnetosphere: Magnetosphere::Absent,
            },
        }
    }
}
