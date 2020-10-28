use super::*;
use crate::body::Bodies;
use crate::colony::{Colonies, Colony};
use crate::ships::freighter_state::{FreighterState, IdleRow};
use crate::star::Stars;
use crate::time::TimeState;
use drives::Drive;

pub mod drives;
pub mod freighter_state;

#[derive(Debug, Default)]
pub struct Shipping {
    pub freighters: Freighters,
}

#[derive(Debug, Clone)]
pub struct Freighter {
    pub tonnage: Mass,
    pub capacity: Mass,
    pub drive: Drive,
}

pub struct FreighterLinks {
    pub location: Id<Colony>,
}

dynamic_arena!(Freighter);

#[derive(Debug, Default)]
pub struct Freighters {
    pub alloc: Allocator<Freighter>,

    pub tonnage: Component<Freighter, Mass>,
    pub capacity: Component<Freighter, Mass>,
    pub drive: Component<Freighter, Drive>,

    pub storage: ResourceComponent<Freighter, Mass>,

    pub state: FreighterState,
}

impl Freighters {
    pub fn create(&mut self, freighter: Freighter, links: FreighterLinks) -> Id<Freighter> {
        let id = self.alloc.create();

        self.tonnage.insert(id, freighter.tonnage);
        self.capacity.insert(id, freighter.capacity);
        self.drive.insert(id, freighter.drive);

        self.storage.insert(id, Mass::zero());

        self.state.insert_row(
            id,
            IdleRow {
                id: id.id(),
                location: links.location,
            },
        );

        id.id()
    }

    pub fn kill(&mut self, id: Id<Freighter>) {
        if let Some(id) = self.alloc.validate(id) {
            self.tonnage.insert(id, Mass::zero());
            self.capacity.insert(id, Mass::zero());
            self.drive.insert(id, Drive::Warp(Speed::zero()));

            self.storage.insert(id, Mass::zero());

            self.state.remove(id);

            let id = id.id();
            self.alloc.kill(id);
        }
    }

    pub fn update(&mut self, time: &TimeState) {
        self.state.transition(time);
    }
}
