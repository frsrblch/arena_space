use crate::*;
use crate::star::Star;

#[derive(Debug, Default)]
pub struct Body {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub mass: Component<Self, Mass>,
    pub radius: Component<Self, Length>,
    pub orbit: Component<Self, Orbit>,

    pub star: Component<Self, Id<Star>>,
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

        let orbit = self.get_orbit(link.parent, row.orbit);
        self.orbit.insert(id, orbit);

        id
    }

    fn get_orbit(&self, parent: Option<Id<Body>>, params: OrbitParams) -> Orbit {
        let parent = parent
            .and_then(|parent| self.orbit.get(parent))
            .map(|orbit| {
                assert!(orbit.parent.is_none());
                orbit.params
            });

        Orbit {
            params,
            parent,
        }
    }

    pub fn get_position(&self, id: Id<Body>, time: Time) -> Option<Position> {
        self.orbit
            .get(id)
            .map(|orbit| orbit.calculate_position(time))
    }
}

#[derive(Debug)]
pub struct BodyRow {
    pub name: String,
    pub mass: Mass,
    pub radius: Length,
    pub orbit: OrbitParams,
}

#[derive(Debug)]
pub struct BodyLinks {
    star: Id<Star>,
    parent: Option<Id<Body>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Orbit {
    pub params: OrbitParams,
    pub parent: Option<OrbitParams>,
}

impl Orbit {
    pub fn calculate_position(&self, time: Time) -> Position {
        if let Some(parent) = self.parent {
            parent.calculate_position(time) + self.params.calculate_position(time)
        } else {
            self.params.calculate_position(time)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OrbitParams {
    pub radius: Length,
    pub period: Time,
    pub offset: Angle,
}

impl OrbitParams {
    pub fn calculate_position(&self, time: Time) -> Position {
        let angle = self.get_angle(time);
        Position::from_angle_and_radius(angle, self.radius)
    }

    pub fn get_angle(&self, time: Time) -> Angle {
        Angle::in_rad(time / self.period * Self::NEG_TWO_PI) - self.offset
    }

    const NEG_TWO_PI: f64 = std::f64::consts::PI * -2.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn orbit_test_at_time_zero() {
        let (bodies, id) = get_one_planet();

        let zero = bodies.get_position(id, Time::in_s(0.0)).unwrap();

        assert_eq!(Position::in_m(0.0, 1000.0), zero);
    }

    #[test]
    fn orbit_test_at_quarter_orbit() {
        let (bodies, id) = get_one_planet();

        let quarter = bodies.get_position(id, Time::in_s(15.0)).unwrap();

        assert_eq!(Length::in_m(-1000.0), quarter.x);
    }

    #[test]
    fn orbit_test_at_zero_with_quarter_offset() {
        let (bodies, id) = get_one_planet_with_offset(Angle::in_rad(PI / 2.0));

        let quarter = bodies.get_position(id, Time::in_s(0.0)).unwrap();

        assert_eq!(Length::in_m(-1000.0), quarter.x);
    }

    #[test]
    fn orbit_test_at_half_orbit() {
        let (bodies, id) = get_one_planet();

        let half = bodies.get_position(id, Time::in_s(30.0)).unwrap();

        assert_eq!(Length::in_m(-1000.0), half.y);
        assert!(Length::in_m(0.001) > half.x && half.x > Length::in_m(-0.001));
    }

    #[test]
    fn orbit_test_at_three_quarter_orbit() {
        let (bodies, id) = get_one_planet();

        let three_quarters = bodies.get_position(id, Time::in_s(45.0)).unwrap();

        assert_eq!(Length::in_m(1000.0), three_quarters.x);
    }

    #[test]
    fn get_moon_orbit() {
        let (bodies, _planet, moon) = get_planet_and_moon();

        let mooon_position = bodies.get_position(moon, Time::in_s(0.0)).unwrap();

        assert_eq!(Position::in_m(0.0, 1010.0), mooon_position);
    }

    fn get_one_planet() -> (Body, Id<Body>) {
        get_one_planet_with_offset(Angle::default())
    }

    fn get_one_planet_with_offset(offset: Angle) -> (Body, Id<Body>) {
        let mut body = Body::default();

        let planet = BodyRow {
            name: "Planet".to_string(),
            mass: Mass::in_kg(100.0),
            radius: Length::in_m(1.0),
            orbit: OrbitParams {
                radius: Length::in_m(1000.0),
                period: Time::in_s(60.0),
                offset,
            }
        };

        let links = BodyLinks {
            star: get_star(),
            parent: None
        };

        let planet = body.create(planet, links);

        (body, planet)
    }

    fn get_planet_and_moon() -> (Body, Id<Body>, Id<Body>) {
        let (mut body, planet) = get_one_planet();

        let moon = BodyRow {
            name: "Moon".to_string(),
            mass: Mass::in_kg(5.0),
            radius: Length::in_m(0.1),
            orbit: OrbitParams {
                radius: Length::in_m(10.0),
                period: Time::in_s(10.0),
                offset: Default::default()
            }
        };

        let links = BodyLinks {
            star: get_star(),
            parent: Some(planet),
        };

        let moon = body.create(moon, links);

        (body, planet, moon)
    }

    fn get_star() -> Id<Star> {
        Allocator::<Star>::default().create()
    }
}