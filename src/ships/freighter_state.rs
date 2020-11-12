use super::Freighter;
use crate::colony::{Colonies, Colony};
use crate::components::{Mass, ResourceComponent, TimeFloat};
use crate::time::TimeState;
use crate::Resource;
use gen_id::*;
use iter_context::ContextualIterator;

table_array! {
    struct FreighterState {
        type Arena = Freighter;
        type RowEnum = enum FreighterStateEnum;
        type IndexEnum = enum FreighterStateIndex;
        tables {
            idle: struct Idle {
                type Row = struct IdleRow;
                fields {}
                links {
                    location: Colony,
                }
            },
            loading: struct Loading {
                type Row = struct LoadingRow;
                fields {
                    resource: Resource,
                }
                links {
                    location: Colony,
                    destination: Colony,
                }
            },
            unloading: struct Unloading {
                type Row = struct UnloadingRow;
                fields {
                    resource: Resource,
                }
                links {
                    location: Colony,
                    destination: Colony,
                }
            },
            moving: struct Moving {
                type Row = struct MovingRow;
                fields {
                    departure: TimeFloat,
                    arrival: TimeFloat,
                    resource: Resource,
                }
                links {
                    from: Colony,
                    to: Colony,
                }
            },
        }
        transitions {
            moving_to_unloading: MovingToUnloading,
        }
    }
}

impl FreighterState {
    pub fn transition(&mut self, time: &TimeState) {
        let time = time.get_time_float();

        let moving = &mut self.moving;
        let unloading = &mut self.unloading;
        let indices = &mut self.indices;

        self.moving_to_unloading
            .transition(time, moving, unloading, indices);
    }
}

#[derive(Debug, Default)]
pub struct MovingToUnloading {
    transition: Vec<Index<Moving>>,
}

impl From<MovingRow> for UnloadingRow {
    fn from(value: MovingRow) -> Self {
        Self {
            id: value.id,
            location: value.to,
            destination: value.from,
            resource: value.resource,
        }
    }
}

impl MovingToUnloading {
    pub fn transition(
        &mut self,
        time: TimeFloat,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut IdIndices<Freighter, FreighterStateIndex>,
    ) {
        self.get_arrivals(&moving.arrival, time);
        self.transition_arrivals(moving, unloading, indices);
    }

    fn get_arrivals(&mut self, arrival: &Column<Moving, TimeFloat>, time: TimeFloat) {
        let iter = arrival
            .iter()
            .zip(arrival.indices())
            .into_iter()
            .filter_map(|(arrival, id)| if *arrival < time { Some(id) } else { None });

        self.transition.clear();
        self.transition.extend(iter);
    }

    fn transition_arrivals(
        &mut self,
        moving: &mut Moving,
        idle: &mut Unloading,
        indices: &mut IdIndices<Freighter, FreighterStateIndex>,
    ) {
        for index in self.drain_rev() {
            let arrived = moving.swap_remove(index, indices);

            let id = Valid::assert(arrived.id);
            let row = arrived.into();
            idle.insert(id, row, indices);
        }
    }

    fn drain_rev(&mut self) -> impl Iterator<Item = Index<Moving>> + '_ {
        self.transition.drain(..).rev()
    }
}

impl Unloading {
    pub fn update(
        &mut self,
        colonies: &mut Colonies,
        cargo: &mut ResourceComponent<Freighter, Mass>,
    ) {
        let ids = Valid::assert(self.id.iter());
        let location = self.location.validate(&colonies.alloc);

        for (id, location) in ids.zip(location.iter()) {
            if let Some(location) = location {
                let stockpile = colonies.resources.stockpile.iter_mut();
                for (stockpile, cargo) in stockpile.zip(cargo.iter_mut()) {
                    let stockpile = stockpile.get_mut(location);
                    let cargo = cargo.get_mut(id);

                    stockpile.give(cargo);
                }
            }
        }

        // TODO add ship loading rate
        // TODO add spaceport loading rate
        // TODO add id to DoneUnloading struct
    }
}
