use super::*;
use crate::body::{Bodies, Body};
use crate::nation::Nation;
use crate::star::Star;

#[derive(Debug)]
pub struct Spaceport {
    pub orbit: OrbitParams,
}

dynamic_arena!(Spaceport);

#[derive(Debug, Copy, Clone)]
pub struct SpaceportLinks {
    /// The nation, if any, controlling a given spaceport
    pub nation: Option<Id<Nation>>,
    /// Spaceport may orbit a body or the star itself if no body is specified
    pub body: Option<Id<Body>>,
    /// The star system in which a spaceport is located
    pub star: Id<Star>,
}

#[derive(Debug, Default)]
pub struct Spaceports {
    pub alloc: Allocator<Spaceport>,

    pub orbit: Component<Spaceport, OrbitParams>,

    pub food: Component<Spaceport, Mass>,

    pub nation: Component<Spaceport, Option<Id<Nation>>>,
    pub body: Component<Spaceport, Option<Id<Body>>>,
    pub star: Component<Spaceport, Id<Star>>,
}

impl Spaceports {
    pub fn create(&mut self, spaceport: Spaceport, links: SpaceportLinks) -> Id<Spaceport> {
        let id = self.alloc.create();

        self.orbit.insert(id, spaceport.orbit);

        self.food.insert(id, Mass::zero());

        self.nation.insert(id, links.nation);
        self.body.insert(id, links.body);
        self.star.insert(id, links.star);

        id.id
    }

    pub fn get_position(
        &self,
        spaceport: Id<Spaceport>,
        time: TimeFloat,
        bodies: &Bodies,
    ) -> Option<Position> {
        self.alloc
            .validate(spaceport)
            .map(|spaceport| self.get_position_validated(spaceport, time, bodies))
    }

    pub fn get_position_validated(
        &self,
        spaceport: Valid<Spaceport>,
        time: TimeFloat,
        bodies: &Bodies,
    ) -> Position {
        let body_position = self.get_body_position(spaceport, time, bodies);
        let relative_position = self.get_relative_position(spaceport, time);

        body_position + relative_position
    }

    fn get_body_position(
        &self,
        spaceport: Valid<Spaceport>,
        time: TimeFloat,
        bodies: &Bodies,
    ) -> Position {
        self.body
            .get(spaceport)
            .map(|body| bodies.get_position(body, time))
            .unwrap_or_default()
    }

    fn get_relative_position(&self, spaceport: Valid<Spaceport>, time: TimeFloat) -> Distance {
        self.orbit
            .get(spaceport)
            .calculate_position(time)
    }
}
