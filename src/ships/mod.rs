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

#[derive(Debug, Default)]
pub struct Shipping {
    pub freighters: Freighters,
}

#[derive(Debug, Clone)]
pub struct Freighter {
    pub tonnage: Mass,
    pub capacity: Mass,
    pub loading_rate: MassRate,
    pub drive: Drive,
}

pub struct FreighterLinks<'c> {
    pub location: Valid<'c, Id<Colony>>,
}

dynamic_arena!(Freighter);

#[derive(Debug, Default)]
pub struct Freighters {
    pub alloc: Allocator<Freighter>,

    pub tonnage: Component<Freighter, Mass>,
    pub capacity: Component<Freighter, Mass>,
    pub loading_rate: Component<Freighter, MassRate>,
    pub drive: Component<Freighter, Drive>,

    pub cargo: Component<Freighter, Vec<CargoEntry>>,

    pub assignment: Component<Freighter, Assignment>,
    pub state: FreighterState,
}

impl Freighters {
    pub fn create(&mut self, freighter: Freighter, links: FreighterLinks) -> Id<Freighter> {
        let id = self.alloc.create();

        self.tonnage.insert(id, freighter.tonnage);
        self.capacity.insert(id, freighter.capacity);
        self.loading_rate.insert(id, freighter.loading_rate);
        self.drive.insert(id, freighter.drive);

        self.cargo.insert(id, Vec::default());

        self.assignment.insert(id, Assignment::None);
        let idle = IdleRow::new(id, links.location);
        self.state.insert(id, idle);

        id.id()
    }

    pub fn kill(&mut self, id: Id<Freighter>) {
        if let Some(id) = self.alloc.validate(id) {
            self.tonnage.insert(id, Mass::zero());
            self.capacity.insert(id, Mass::zero());
            self.loading_rate.insert(id, MassRate::zero());
            self.drive.insert(id, Drive::Warp(Speed::zero()));

            self.cargo.get_mut(id).clear();

            self.assignment.insert(id, Assignment::None);
            self.state.remove(id);

            let id = id.id();
            self.alloc.kill(id);
        }
    }

    pub fn update(&mut self, time: &TimeState, colonies: &Colonies) {
        self.state.transition(time, colonies, &self.assignment);
    }
}
