use super::*;
// use crate::body::{Bodies, Body};
// use crate::nation::Nation;
// use crate::star::Star;
// use crate::colony::{Colony, Colonies};
// use crate::colony::production::SpaceportConstruction;

#[derive(Debug, Copy, Clone)]
pub struct Spaceport {
    pub mass: Mass,
    pub orbit: OrbitParams,
}

impl Spaceport {
    pub fn construction_rate() -> MassRate {
        10.0 * TON / DAY
    }
}

// dynamic_arena!(Spaceport);
//
// #[derive(Debug, Copy, Clone)]
// pub struct SpaceportLinks {
//     /// The nation, if any, controlling a given spaceport
//     pub nation: Option<Id<Nation>>,
//     /// Spaceport may orbit a body or the star itself if no body is specified
//     pub body: Option<Id<Body>>,
//     /// The star system in which a spaceport is located
//     pub star: Id<Star>,
// }
//
// impl SpaceportLinks {
//     pub fn from_colony<I: Indexes<Colony>>(id: I, colonies: &Colonies, bodies: &Bodies) -> Self {
//         let nation = colonies.nation.get(id).as_ref().copied();
//         let body = *colonies.body.get(id);
//         let star = *bodies.star.get(body);
//
//         SpaceportLinks {
//             nation,
//             body: Some(body),
//             star,
//         }
//     }
// }
//
// #[derive(Debug, Default)]
// pub struct Spaceports {
//     pub alloc: Allocator<Spaceport>,
//
//     pub mass: Component<Spaceport, Mass>,
//     pub orbit: Component<Spaceport, OrbitParams>,
//
//     pub nation: Component<Spaceport, Option<Id<Nation>>>,
//     pub body: Component<Spaceport, Option<Id<Body>>>,
//     pub star: Component<Spaceport, Id<Star>>,
// }
//
// impl Spaceports {
//     pub fn create_all(&mut self, production: &mut SpaceportConstruction) {
//
//     }
//
//     pub fn create(&mut self, spaceport: Spaceport, links: SpaceportLinks) -> Id<Spaceport> {
//         let id = self.alloc.create();
//
//         self.mass.insert(id, spaceport.mass);
//         self.orbit.insert(id, spaceport.orbit);
//
//         self.nation.insert(id, links.nation);
//         self.body.insert(id, links.body);
//         self.star.insert(id, links.star);
//
//         id.id
//     }
//
//     pub fn get_position(
//         &self,
//         spaceport: Id<Spaceport>,
//         time: TimeFloat,
//         bodies: &Bodies,
//     ) -> Option<Position> {
//         self.alloc
//             .validate(spaceport)
//             .map(|spaceport| self.get_position_validated(spaceport, time, bodies))
//     }
//
//     pub fn get_position_validated(
//         &self,
//         spaceport: Valid<Spaceport>,
//         time: TimeFloat,
//         bodies: &Bodies,
//     ) -> Position {
//         let body_position = self.get_body_position(spaceport, time, bodies);
//         let relative_position = self.get_relative_position(spaceport, time);
//
//         body_position + relative_position
//     }
//
//     fn get_body_position(
//         &self,
//         spaceport: Valid<Spaceport>,
//         time: TimeFloat,
//         bodies: &Bodies,
//     ) -> Position {
//         self.body
//             .get(spaceport)
//             .map(|body| bodies.get_position(body, time))
//             .unwrap_or_default()
//     }
//
//     fn get_relative_position(&self, spaceport: Valid<Spaceport>, time: TimeFloat) -> Distance {
//         self.orbit.get(spaceport).calculate_position(time)
//     }
// }
