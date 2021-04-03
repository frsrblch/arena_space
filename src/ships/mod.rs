use super::*;
use crate::body::Bodies;
use crate::colony::{Colonies, Colony};
use crate::star::Stars;
use crate::time::TimeState;

use crate::ships::cargo::CargoEntry;
use drives::*;
use freighter_assignment::*;
use freighter_state::*;

pub mod cargo;
pub mod drives;
pub mod freighter_assignment;
pub mod freighter_state;

mod redo;

#[derive(Debug, Default)]
pub struct Shipping {
    pub freighters: Freighters,
}

#[derive(Debug, Clone)]
pub struct Freighter {
    pub name: String,
    pub tonnage: Mass,
    pub capacity: Mass,
    pub loading_rate: MassRate,
    pub shipping_cost: PricePerMeter,
    pub drive: Drive,
}

pub struct FreighterLinks {
    pub location: Id<Colony>,
}

dynamic_arena!(Freighter);

#[derive(Debug, Default)]
pub struct Freighters {
    pub alloc: Allocator<Freighter>,

    pub name: Component<Freighter, String>,
    pub tonnage: Component<Freighter, Mass>,
    pub capacity: Component<Freighter, Mass>,
    pub loading_rate: Component<Freighter, MassRate>,
    pub shipping_cost: Component<Freighter, PricePerMeter>,
    pub drive: Component<Freighter, Drive>,

    pub cargo: Component<Freighter, Vec<CargoEntry>>,

    pub assignment: Component<Freighter, Option<Assignment>>,
    pub state: FreighterState,
}

impl Freighters {
    pub fn create(&mut self, freighter: Freighter, links: FreighterLinks) -> Id<Freighter> {
        let id = self.alloc.create();

        self.name.insert(id, freighter.name);
        self.tonnage.insert(id, freighter.tonnage);
        self.capacity.insert(id, freighter.capacity);
        self.loading_rate.insert(id, freighter.loading_rate);
        self.shipping_cost.insert(id, freighter.shipping_cost);
        self.drive.insert(id, freighter.drive);

        self.cargo.insert(id, Vec::default());

        self.assignment.insert(id, None);
        let idle = IdleRow::new(links.location);
        self.state.insert(id, idle);

        id.id()
    }

    pub fn kill(&mut self, id: Id<Freighter>) {
        if let Some(id) = self.alloc.validate(id) {
            self.tonnage.insert(id, Mass::zero());
            self.capacity.insert(id, Mass::zero());
            self.loading_rate.insert(id, MassRate::zero());
            self.shipping_cost.insert(id, PricePerMeter::zero());
            self.drive.insert(id, Drive::Warp(Speed::zero()));

            self.cargo.get_mut(id).clear();

            self.assignment.insert(id, None);
            self.state.remove(id);

            let id = id.id();
            self.alloc.kill(id);
        }
    }

    pub fn update(
        &mut self,
        time: &TimeState,
        colonies: &mut Colonies,
        bodies: &Bodies,
        stars: &Stars,
    ) {
        let parameters = &mut Parameters {
            assignment: &mut self.assignment,
            cargo: &mut self.cargo,
            loading_rate: &self.loading_rate,
            shipping_cost: &self.shipping_cost,
            capacity: &self.capacity,
            drive: &self.drive,
            time,
            colonies,
            bodies,
            stars,
        };

        self.state.update(parameters);
    }

    // pub fn get_position<F: ValidId<Freighter>, S: ValidId<Star>>(
    //     &self,
    //     id: F,
    //     star: S,
    //     stars: &Stars,
    //     bodies: &Bodies,
    //     colonies: &Colonies,
    //     time: &TimeState,
    // ) -> Position {
    //     let state = self.state.indices().get(id);
    //     match state {
    //         FreighterStateIndex::Idle(index) => {
    //             let colony
    //         }
    //         FreighterStateIndex::Loading(_) => todo!(),
    //         FreighterStateIndex::Unloading(_) => todo!(),
    //         FreighterStateIndex::Moving(i) => {
    //             let moving = &self.state.moving;
    //
    //             let src = moving.source.get(i);
    //             let src_body = colonies.body.get(src);
    //             let src_star = bodies.star.get(src_body);
    //
    //             let dest = moving.destination.get(i);
    //             let dest_body = colonies.body.get(dest);
    //             let dest_star = bodies.star.get(dest_body);
    //
    //             if star.id() == *src_star && star.id() == *dest_star {
    //                 let departure = *moving.departure.get(i)?;
    //                 let arrival = *moving.arrival.get(i)?;
    //
    //                 let departure_pos = bodies.get_position(src_body, departure);
    //                 let arrival_pos = bodies.get_position(dest_body, arrival);
    //                 let trip_vector = arrival_pos - departure_pos;
    //
    //                 let fraction = moving.get_trip_fraction(i, time)?;
    //
    //                 departure_pos + trip_vector * fraction.value();
    //             } else {
    //                 let _ = stars;
    //                 todo!()
    //             }
    //         }
    //     }
    // }
}
