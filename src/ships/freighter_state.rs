use super::Freighter;
use crate::colony::{Colonies, Colony};
use crate::components::{MassRate, TimeFloat};
use crate::ships::cargo::CargoEntry;
use crate::ships::freighter_assignment::Assignment;
use crate::systems::System;
use crate::time::TimeState;
use crate::Resource;
use gen_id::*;
use iter_context::ContextualIterator;
use std::iter::Rev;
use std::vec::Drain;

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
                fields {}
                links {
                    location: Colony,
                    destination: Colony,
                }
            },
            unloading: struct Unloading {
                type Row = struct UnloadingRow;
                fields {
                }
                links {
                    location: Colony,
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
                    source: Colony,
                    destination: Colony,
                }
            },
        }
        transitions {
            transition_moving: TransitionMoving,
        }
    }
}

impl FreighterState {
    pub fn transition(
        &mut self,
        time: &TimeState,
        colonies: &Colonies,
        assignment: &Component<Freighter, Assignment>,
    ) {
        let time = time.get_time_float();

        let moving = &mut self.moving;
        let unloading = &mut self.unloading;
        let indices = &mut self.indices;

        self.transition_moving
            .transition(time, moving, unloading, indices, colonies, assignment);
    }
}

#[derive(Debug, Default)]
pub struct TransitionMoving {
    transition: Vec<Index<Moving>>,
}

impl From<MovingRow> for UnloadingRow {
    fn from(value: MovingRow) -> Self {
        Self {
            id: value.id,
            location: value.destination,
        }
    }
}

impl TransitionMoving {
    pub fn transition(
        &mut self,
        time: TimeFloat,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut IdIndices<Freighter, FreighterStateIndex>,
        colonies: &Colonies,
        assignment: &Component<Freighter, Assignment>,
    ) {
        self.get_arrivals(&moving.arrival, time);
        self.transition_arrivals(moving, unloading, indices, colonies, assignment);
    }

    fn get_arrivals(&mut self, arrival: &Column<Moving, TimeFloat>, time: TimeFloat) {
        let iter = arrival
            .iter()
            .zip(arrival.indices())
            .into_iter()
            .filter_map(
                |(arrival, id)| {
                    if *arrival < time {
                        Some(id)
                    } else {
                        None
                    }
                },
            );

        self.transition.clear();
        self.transition.extend(iter);
    }

    fn transition_arrivals(
        &mut self,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut IdIndices<Freighter, FreighterStateIndex>,
        colonies: &Colonies,
        _assignment: &Component<Freighter, Assignment>,
    ) {
        for index in self.rev_drain() {
            let arrived = moving.swap_remove(index, indices);

            if let Some(_) = arrived
                .destination
                .and_then(|id| id.validate(&colonies.alloc))
            {
                let row = Valid::assert(arrived.into());
                unloading.insert(row, indices);
            } else {
                todo!("Destination not available, reroute ship");
            }
        }
    }

    fn rev_drain(&mut self) -> Rev<Drain<Index<Moving>>> {
        self.transition.drain(..).rev()
    }
}

impl Unloading {
    pub fn update(
        &mut self,
        colonies: &mut Colonies,
        cargo_manifest: &mut Component<Freighter, Vec<CargoEntry>>,
        loading_rate: &Component<Freighter, MassRate>,
        done: &mut DoneUnloading,
    ) {
        let stockpile = &mut colonies.resources.stockpile;

        let ids = Valid::assert(self.id.iter());
        let location = self.location.validate(&colonies.alloc);

        let interval = System::FreighterState.get_interval_float();

        for ((id, location), index) in ids.zip(location.iter()).zip(self.id.indices()) {
            if let Some(location) = location {
                let mut unload_capacity = loading_rate.get(id) * interval;

                let cargo_manifest = cargo_manifest.get_mut(id);

                while !cargo_manifest.is_empty() && !unload_capacity.is_none() {
                    let CargoEntry { resource, amount } = cargo_manifest.last_mut().unwrap();

                    let unloaded = amount.request(unload_capacity);
                    unload_capacity -= unloaded;

                    let colony_amount = stockpile.get_mut(*resource).get_mut(location);

                    *colony_amount += unloaded;

                    if amount.is_none() {
                        cargo_manifest.pop();
                    }
                }

                if cargo_manifest.is_empty() {
                    done.push(index);
                }
            } else {
                done.push(index);
            }
        }

        // TODO add spaceport loading rate
        // TODO add id to DoneUnloading struct
    }
}

pub struct DoneUnloading {
    done: Vec<Index<Unloading>>,
}

impl DoneUnloading {
    fn push(&mut self, index: Index<Unloading>) {
        self.done.push(index);
    }

    pub fn transition(
        &mut self,
        assignment: &mut Component<Freighter, Assignment>,
        unloading: &mut Unloading,
        loading: &mut Loading,
        idling: &mut Idle,
        indices: &mut IdIndices<Freighter, FreighterStateIndex>,
        colonies: &Colonies,
    ) {
        for index in self.rev_drain() {
            let row = unloading.swap_remove(index, indices);
            let row = Valid::assert(row);

            let id = UnloadingRow::id(&row);
            if let Some(location) = UnloadingRow::location(&row) {
                match assignment.get(id) {
                    Assignment::None => {
                        let idle = IdleRow::new(id, location);
                        idling.insert(idle, indices);
                    }
                    Assignment::Route(a, b) => {
                        if let (Some(a), Some(b)) =
                            (a.validate(&colonies.alloc), b.validate(&colonies.alloc))
                        {
                            let destination = if location == a {
                                b
                            } else if location == b {
                                a
                            } else {
                                panic!()
                            };

                            let row = LoadingRow::new(id, location, destination);
                            loading.insert(row, indices);
                        } else {
                            let idle = IdleRow::new(id, location);
                            idling.insert(idle, indices);
                        }
                    }
                }
            } else {
            }
        }
    }

    fn rev_drain(&mut self) -> Rev<Drain<Index<Unloading>>> {
        self.done.drain(..).rev()
    }
}
