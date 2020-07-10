use crate::*;
use crate::body::Planet;

#[derive(Debug, Clone)]
pub struct Star {
    pub name: String,
    pub position: Position,
    pub properties: StarProperties,
}

fixed_arena!(Star);

#[derive(Debug, Default)]
pub struct Stars {
    pub alloc: Allocator<Star>,

    pub name: Component<Star, String>,
    pub position: Component<Star, Position>,
    pub properties: Component<Star, StarProperties>,
}

impl Stars {
    pub fn create(&mut self, row: Star) -> Id<Star> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.position.insert(id, row.position);
        self.properties.insert(id, row.properties);

        id
    }
}

#[derive(Debug, Clone)]
pub struct StarSystem {
    star: Star,
    planets: Vec<Planet>,
}

impl State {
    pub fn create(&mut self, star_system: StarSystem) {
        let star = self.star.create(star_system.star);

        for planet in star_system.planets {
            self.body.create_planet(planet, star);
        }
    }
}

// star generation
// simplest to randomly generate star type based on observed star distribution, and pick random attributes based on the type
#[derive(Debug, Copy, Clone)]
pub struct StarProperties {
    pub classification: StarClassification,
    pub fraction: Fraction,
}

impl StarProperties {
    pub fn g(fraction: Fraction) -> Self {
        StarProperties {
            classification: StarClassification::G,
            fraction,
        }
    }

    pub fn get_temperature(self) -> Temperature {
        let kelvin = match self.classification {
            StarClassification::G => 5e3 + self.fraction * 1e3,
        };

        Temperature::in_k(kelvin)
    }

    pub fn get_radius(self) -> Length {
        let solar_fraction: f64 = match self.classification {
            StarClassification::G => 0.85 + self.fraction * 0.3,
        };

        SOLAR_RADIUS * solar_fraction
    }

    pub fn get_mass(self) -> Mass {
        let solar_fraction: f64 = match self.classification {
            StarClassification::G => 0.85 + self.fraction * 0.3,
        };

        SOLAR_MASS * solar_fraction
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StarClassification {
    G
}

const SOLAR_MASS: Mass = Mass::in_kg(1.9884e30);
const SOLAR_RADIUS: Length = Length::in_m(695_700e3);