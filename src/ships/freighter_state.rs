use super::Freighter;
use crate::body::Bodies;
use crate::colony::{Colonies, Colony};
use crate::components::{Duration, Mass, MassRate, Price, Resource, ResourceArray, TimeFloat};
use crate::ships::cargo::CargoEntry;
use crate::ships::drives::Drive;
use crate::ships::freighter_assignment::Assignment;
use crate::star::Stars;
use crate::systems::System;
use crate::time::TimeState;
use gen_id::*;
use iter_context::ContextualIterator;

const INTERVAL: Duration = System::FreighterState.get_interval();

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
                    abort: bool,
                }
                links {
                    location: Colony,
                    destination: Colony,
                }
            },
            unloading: struct Unloading {
                type Row = struct UnloadingRow;
                fields {
                    arrival: TimeFloat,
                    completion: TimeFloat,
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
                }
                links {
                    source: Colony,
                    destination: Colony,
                }
            },
        }
        transitions {
            assign: Assign,
            arrivals: Arrivals,
            unloaded: Unloaded,
            loaded: Loaded,
        }
    }
}

type Indices = IdIndices<Freighter, FreighterStateIndex>;

pub struct Parameters<'a> {
    pub assignment: &'a mut Component<Freighter, Option<Assignment>>,
    pub cargo: &'a mut Component<Freighter, Vec<CargoEntry>>,
    pub capacity: &'a Component<Freighter, Mass>,
    pub loading_rate: &'a Component<Freighter, MassRate>,
    pub drive: &'a Component<Freighter, Drive>,

    pub time: &'a TimeState,
    pub stars: &'a Stars,
    pub bodies: &'a Bodies,
    pub colonies: &'a mut Colonies,
}

impl<'a> Parameters<'a> {
    fn get_unloading_duration<I: ValidId<Freighter>>(&self, id: I) -> Duration {
        let contents = self.cargo.get(id).iter().map(|c| c.amount).sum::<Mass>();
        let loading_rate = self.loading_rate.get(id);

        contents / loading_rate
    }

    fn get_trip_duration<F: ValidId<Freighter>, C: ValidId<Colony>>(
        &self,
        id: F,
        from: C,
        to: C,
    ) -> Duration {
        let drive = self.drive.get(id);
        let time = self.time.get_time_float();

        drive.calculate_trip_duration(from, to, time, self.colonies, self.bodies, self.stars)
    }

    fn contents<F: ValidId<Freighter>>(&self, id: F) -> Mass {
        self.cargo.get(id).iter().map(|c| c.amount).sum::<Mass>()
    }

    fn is_cargo_full<F: ValidId<Freighter>>(&self, id: F) -> bool {
        self.contents(id) >= *self.capacity.get(id)
    }

    fn get_price_gradient<I: ValidId<Colony>>(
        &self,
        source: I,
        destination: I,
        resource: Resource,
    ) -> Price {
        let prices = &self.colonies.resources.price.get(resource);
        let destination_price = prices.get(destination);
        let source_price = prices.get(source);
        destination_price - source_price
    }

    fn get_stockpile<C: ValidId<Colony>>(&self, colony: C, resource: Resource) -> Mass {
        *self.colonies.resources.stockpile.get(resource).get(colony)
    }

    fn get_stockpile_mut<C: ValidId<Colony>>(
        &mut self,
        colony: C,
        resource: Resource,
    ) -> &mut Mass {
        self.colonies
            .resources
            .stockpile
            .get_mut(resource)
            .get_mut(colony)
    }

    fn add_cargo<F: ValidId<Freighter>>(&mut self, id: F, resource: Resource, amount: Mass) {
        let cargo = self.cargo.get_mut(id);

        if let Some(entry) = cargo.iter_mut().find(|e| e.resource == resource) {
            entry.amount += amount;
        } else {
            cargo.push(CargoEntry { resource, amount });
        }
    }
}

impl FreighterState {
    pub fn update(&mut self, parameters: &mut Parameters) {
        let idle = &mut self.idle;
        let assign = &mut self.assign;

        let moving = &mut self.moving;
        let arrivals = &mut self.arrivals;

        let loading = &mut self.loading;
        let loaded = &mut self.loaded;

        let unloading = &mut self.unloading;
        let unloaded = &mut self.unloaded;

        let indices = &mut self.indices;

        // do updates before transitions
        // otherwise, a new arrival would spend the hour unloading
        loading.update(parameters);

        // each transition can lead into the next if applicable
        arrivals.transition(moving, unloading, indices, parameters);
        unloaded.transition(unloading, idle, indices, parameters);
        assign.transition(idle, moving, loading, indices, parameters);
        loaded.transition(loading, moving, unloading, indices, parameters);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Assign {
    assign: Transition<(Index<Idle>, Assignment)>,
}

impl Assign {
    pub fn transition(
        &mut self,
        idle: &mut Idle,
        moving: &mut Moving,
        loading: &mut Loading,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        self.get_idle_assigned(idle, parameters);
        self.transition_assigned(idle, moving, loading, indices, parameters);
    }

    fn get_idle_assigned(&mut self, idle: &mut Idle, parameters: &Parameters) {
        let ids = idle.id.iter().map(Valid::assert);

        let iter = ids
            .zip(idle.indices())
            .into_iter()
            .filter_map(|(id, index)| match parameters.assignment.get(id) {
                None => None,
                Some(assignment) => Some((index, *assignment)),
            });

        self.assign.fill(iter);
    }

    fn transition_assigned(
        &mut self,
        idle: &mut Idle,
        moving: &mut Moving,
        loading: &mut Loading,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        let time = parameters.time.get_time_float();

        for (index, assignment) in self.assign.drain() {
            let (id, idle_row) = idle.swap_remove(index, indices);
            let id = Valid::assert(id);

            assert!(
                parameters.cargo.get_mut(id).is_empty(),
                "Colony shipping system assumes that freighters are empty when they start loading"
            );

            match assignment {
                Assignment::Route(a, b) => {
                    if idle_row.location.eq(&a) {
                        let row = LoadingRow::new(false, idle_row.location, b);
                        loading.insert(id, row, indices);
                    } else if idle_row.location.eq(&b) {
                        let row = LoadingRow::new(false, idle_row.location, a);
                        loading.insert(id, row, indices);
                    } else {
                        // go to nearest
                        let to_a = parameters.get_trip_duration(id, idle_row.location, a);
                        let to_b = parameters.get_trip_duration(id, idle_row.location, b);

                        let (destination, duration) =
                            if to_a > to_b { (b, to_b) } else { (a, to_a) };

                        if duration <= Duration::MAX {
                            let row = MovingRow::new(
                                time,
                                time + duration,
                                idle_row.location,
                                destination,
                            );
                            moving.insert(id, row, indices);
                        } else {
                            // destination unreachable
                            parameters.assignment.insert(id, None);
                            idle.insert(id, Valid::assert(idle_row), indices);
                        }
                    };
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Arrivals {
    transition: Transition<Index<Moving>>,
}

impl Arrivals {
    pub fn transition(
        &mut self,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut Indices,
        parameters: &Parameters,
    ) {
        self.get_arrivals(&moving.arrival, parameters);
        self.transition_arrivals(moving, unloading, indices, parameters);
    }

    fn get_arrivals(&mut self, arrival: &Column<Moving, TimeFloat>, parameters: &Parameters) {
        let time = parameters.time.get_time_float();

        let iter = arrival
            .iter()
            .zip(arrival.indices())
            .into_iter()
            .filter_map(|(arrival, id)| (*arrival < time).then_some(id));

        self.transition.fill(iter);
    }

    fn transition_arrivals(
        &mut self,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut Indices,
        parameters: &Parameters,
    ) {
        self.transition.drain().for_each(|index| {
            // TODO reconfigure to use Valid<Moving>::swap_remove(index, indices) -> Valid<MovingRow>

            let (id, row) = moving.swap_remove(index, indices);
            let id = Valid::assert(id);

            let cargo = parameters.cargo.get(id);
            let cargo = cargo.iter().map(|c| c.amount).sum::<Mass>();
            let loading_rate = parameters.loading_rate.get(id);

            let duration = cargo / loading_rate;
            let complete = row.arrival + duration;

            let row = UnloadingRow::new(row.arrival, complete, row.destination);

            unloading.insert(id, row, indices);
        });
    }
}

#[derive(Debug, Default, Clone)]
pub struct Unloaded {
    transition: Transition<Index<Unloading>>,
}

impl Unloaded {
    pub fn transition(
        &mut self,
        unloading: &mut Unloading,
        idling: &mut Idle,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        self.get_unloaded(unloading, parameters);
        self.transition_unloaded(unloading, idling, indices, parameters);
    }

    fn get_unloaded(&mut self, unloading: &Unloading, parameters: &Parameters) {
        let time = parameters.time.get_time_float();

        let iter = unloading
            .completion
            .iter()
            .zip(unloading.indices())
            .into_iter()
            .filter_map(|(completion, index)| (*completion < time).then_some(index));

        self.transition.fill(iter);
    }

    fn transition_unloaded(
        &mut self,
        unloading: &mut Unloading,
        idling: &mut Idle,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        self.transition.drain().for_each(|index| {
            let (id, row) = unloading.swap_remove(index, indices);
            let id = Valid::assert(id);

            let cargo = parameters.cargo.get_mut(id);
            let stockpile = &mut parameters.colonies.resources.stockpile;
            let shipping = &mut parameters.colonies.resources.shipping;

            for CargoEntry { resource, amount } in cargo.drain(..) {
                let stockpile = stockpile.get_mut(resource).get_mut(row.location);
                let shipping = shipping.get_mut(resource).get_mut(row.location);

                *stockpile += amount;
                *shipping += amount;
            }

            let idle = IdleRow::new(row.location);
            idling.insert(id, idle, indices);
        });
    }
}

impl Loading {
    pub fn update(&mut self, parameters: &mut Parameters) {
        let ids = self.id.iter().map(Valid::assert);

        ids.zip(self.location.iter())
            .zip(self.destination.iter())
            .zip(self.abort.iter_mut())
            .for_each(|(((id, location), destination), abort)| {
                let loading_rate = parameters.loading_rate.get(id);

                let capacity = parameters.capacity.get(id);
                let contents = parameters.contents(id);

                let remaining = capacity - contents;
                let mut to_load = remaining.min(loading_rate * INTERVAL);

                let mut price_difference = ResourceArray::<Price>::default();

                price_difference
                    .iter_enum_mut()
                    .for_each(|(price, resource)| {
                        *price = parameters.get_price_gradient(location, destination, *resource);
                    });

                while to_load.is_some() && !*abort {
                    if let Some((_, resource)) = price_difference
                        .iter_enum()
                        .into_iter()
                        .filter(|(price, resource)| {
                            **price > Price::zero()
                                && parameters.get_stockpile(location, **resource).is_some()
                        })
                        .fold_first(|(max_price, max_resource), (price, resource)| {
                            if price > max_price {
                                (price, resource)
                            } else {
                                (max_price, max_resource)
                            }
                        })
                    {
                        let stock = parameters.get_stockpile_mut(location, *resource);
                        let loaded = stock.request(to_load);

                        to_load -= loaded;

                        parameters.add_cargo(id, *resource, loaded);
                    } else {
                        *abort = true;
                    }
                }
            });
    }
}

#[derive(Debug, Default, Clone)]
pub struct Loaded {
    transition: Transition<Index<Loading>>,
}

impl Loaded {
    pub fn transition(
        &mut self,
        loading: &mut Loading,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        self.get_loaded(loading, parameters);
        self.transition_loaded(loading, moving, unloading, indices, parameters);
    }

    fn get_loaded(&mut self, loading: &Loading, parameters: &Parameters) {
        let ids = loading.id.iter().map(Valid::assert);

        let iter = ids
            .zip(loading.indices())
            .zip(loading.abort.iter())
            .into_iter()
            .filter_map(|((id, index), abort)| {
                (parameters.is_cargo_full(id) || *abort).then_some(index)
            });

        self.transition.fill(iter);
    }

    fn transition_loaded(
        &mut self,
        loading: &mut Loading,
        moving: &mut Moving,
        unloading: &mut Unloading,
        indices: &mut Indices,
        parameters: &mut Parameters,
    ) {
        let time = parameters.time.get_time_float();

        self.transition.drain().for_each(|index| {
            let (id, row) = loading.swap_remove(index, indices);
            let id = Valid::assert(id);

            for cargo in parameters.cargo.get(id) {
                let shipped = &mut parameters.colonies.resources.shipping;
                let shipped = shipped.get_mut(cargo.resource).get_mut(row.location);
                *shipped -= cargo.amount;
            }

            match parameters.assignment.get(id) {
                Some(Assignment::Route(a, destination)) if row.location.eq(a) => {
                    let duration = parameters.get_trip_duration(id, row.location, *destination);
                    let row = MovingRow::new(time, time + duration, row.location, destination);
                    moving.insert(id, row, indices);
                }
                Some(Assignment::Route(destination, b)) if row.location.eq(b) => {
                    let duration = parameters.get_trip_duration(id, row.location, *destination);
                    let row = MovingRow::new(time, time + duration, row.location, destination);
                    moving.insert(id, row, indices);
                }
                Some(Assignment::Route(_, _)) | None => {
                    let duration = parameters.get_unloading_duration(id);
                    let row = UnloadingRow::new(time, time + duration, row.destination);
                    unloading.insert(id, row, indices);
                }
            }
        });
    }
}
