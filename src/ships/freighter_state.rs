use super::Freighter;
use crate::colony::Colony;
use crate::components::TimeFloat;
use crate::time::TimeState;
use crate::Resource;
use gen_id::*;
use typed_iter::TypedIterator;

table_array! {
    struct FreighterState {
        type Arena = Freighter;
        type RowEnum = enum FreighterStateEnum;
        type IndexEnum = enum FreighterStateIndex;
        tables {
            idle: struct Idle {
                type Row = struct IdleRow;
                location: Id<Colony>,
            },
            loading: struct Loading {
                type Row = struct LoadingRow;
                location: Id<Colony>,
                destination: Id<Colony>,
                resource: Resource,
            },
            unloading: struct Unloading {
                type Row = struct UnloadingRow;
                location: Id<Colony>,
                destination: Id<Colony>,
                resource: Resource,
            },
            moving: struct Moving {
                type Row = struct MovingRow;
                from: Id<Colony>,
                to: Id<Colony>,
                departure: TimeFloat,
                arrival: TimeFloat,
                resource: Resource,
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
