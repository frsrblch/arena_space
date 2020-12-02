use crate::body::{Body, Planet};
use crate::state::State;
use crate::*;

// create_fixed_arena! {
//     struct Stars {
//         type Entity = struct Star;
//         components {
//             name: String,
//             position: Position,
//             star_type: StarType,
//         }
//     }
// }

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
}

impl Stars {
    pub fn create(&mut self, row: Star) -> Id<Star> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.position.insert(id, row.position);
        self.star_type.insert(id, row.star_type);

        id
    }
}

#[derive(Debug, Clone)]
pub struct StarSystem {
    pub star: Star,
    pub planets: Vec<Planet>,
}

impl State {
    pub fn create(&mut self, star_system: StarSystem) -> (Id<Star>, Vec<Id<Body>>) {
        let star = self.star.create(star_system.star);

        let planets = star_system
            .planets
            .into_iter()
            .map(|p| self.body.create_planet(p, star))
            .collect();

        (star, planets)
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

const SOLAR_MASS: Mass = 1.9884e30 * KG;
const SOLAR_RADIUS: Length = 695_700.0 * KM;

pub mod examples {
    use super::*;
    use crate::body::examples::*;

    pub fn sol_system() -> StarSystem {
        StarSystem {
            star: sol(),
            planets: vec![planet_earth()],
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
