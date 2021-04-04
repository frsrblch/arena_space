use crate::body::{Body, Planet};
use crate::state::State;
use crate::*;
use iter_context::ContextualIterator;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct Star {
    pub name: String,
    pub position: Position,
    pub star_type: StarType,
}

fixed_arena!(Star);

#[derive(Debug, Default)]
pub struct Stars {
    pub alloc: Allocator<Star>,

    pub name: Component<Star, String>,
    pub position: Component<Star, Position>,
    pub star_type: Component<Star, StarType>,

    bodies: Component<Star, Vec<Id<Body>>>,
}

impl Stars {
    pub fn create(&mut self, row: Star) -> Id<Star> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.position.insert(id, row.position);
        self.star_type.insert(id, row.star_type);
        self.bodies.insert(id, Vec::with_capacity(16));

        id
    }

    #[inline]
    pub fn get_radius<I: ValidId<Star>>(&self, id: I) -> Length {
        self.star_type.get(id).get_radius()
    }

    pub fn get_by_name(&self, name: &str) -> Option<Id<Star>> {
        self.name
            .iter()
            .zip(self.alloc.ids())
            .into_iter()
            .filter_map(
                |(star_name, id)| {
                    if star_name.eq(name) {
                        Some(id)
                    } else {
                        None
                    }
                },
            )
            .nth(0)
    }

    pub fn bodies(&self, star: Id<Star>) -> &Vec<Id<Body>> {
        self.bodies.get(star)
    }
}

#[derive(Debug, Clone)]
pub struct StarSystem {
    pub star: Star,
    pub planets: Vec<Planet>,
}

impl State {
    pub fn create_star_system(&mut self, star_system: StarSystem) {
        let star = self.star.create(star_system.star);
        let bodies = self.star.bodies.get_mut(star);

        for planet in star_system.planets {
            self.body.create_planetary_system(planet, star, bodies);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StarType {
    G(Fraction),
}

impl StarType {
    pub fn get_temperature(self) -> Temperature {
        let kelvin = match self {
            StarType::G(fraction) => 5e3 + fraction * 1e3,
        };

        Temperature::in_k(kelvin)
    }

    pub fn get_radius(self) -> Length {
        let solar_fraction: f64 = match self {
            StarType::G(fraction) => 0.85 + fraction * 0.3,
        };

        SOLAR_RADIUS * solar_fraction
    }

    pub fn get_mass(self) -> Mass {
        let solar_fraction: f64 = match self {
            StarType::G(fraction) => 0.85 + fraction * 0.3,
        };

        SOLAR_MASS * solar_fraction
    }
}

impl Display for StarType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            StarType::G(_) => write!(f, "G-Type star"),
        }
    }
}

const SOLAR_MASS: Mass = 1.9884e30 * KG;
const SOLAR_RADIUS: Length = 695_700.0 * KM;

pub mod examples {
    use super::*;
    use crate::body::examples::*;

    pub fn sol_system() -> StarSystem {
        StarSystem {
            star: sol(),
            planets: vec![mercury(), venus(), earth(), mars()],
        }
    }

    pub fn sol() -> Star {
        Star {
            name: "Sol".to_string(),
            position: Default::default(),
            star_type: StarType::G(Fraction::clamp(0.5)),
        }
    }
}
