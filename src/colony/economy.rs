use crate::components::{Mass, ResourceComponent, FacilityMap, MassRate, DurationFloat, Facility};
use crate::colony::{Colony, Colonies};
use arena_ecs::*;
use crate::systems::System;
use std::slice::{Iter, IterMut};
use std::iter::Zip;

const INTERVAL: DurationFloat = crate::systems::System::ColonyProductionCycle.get_interval_float();

impl Colonies {
    pub fn production_cycle(&mut self) {
        self.request_resources();
        self.set_fulfillment();
        self.read_fulfillment();
        self.take_inputs();
        self.output_production();
    }

    fn request_resources(&mut self) {
        self.resources.reset_requests();
        self.production.request_resouces(&mut self.resources, &self.alloc);
        self.people.request_food(&mut self.resources);
    }

    fn set_fulfillment(&mut self) {
        self.resources.set_fulfillment();
    }

    fn read_fulfillment(&mut self) {
        self.production.get_fulfillment(&self.resources, &self.alloc);
    }

    fn take_inputs(&mut self) {
        self.production.take_inputs(&mut self.resources, &self.alloc);
        self.people.take_food(&mut self.resources);
        self.resources.set_negatives_to_zero();
    }

    fn output_production(&mut self) {
        self.production.output(&mut self.resources, &self.alloc);
    }
}

#[derive(Debug, Default)]
pub struct Resources {
    pub stockpile: ResourceComponent<Colony, Mass>,
    pub requested: ResourceComponent<Colony, MassRate>,
    pub fulfillment: ResourceComponent<Colony, f64>,
}

impl Resources {
    pub fn insert<I: ValidId<Colony>>(&mut self, id: I) {
        self.stockpile.insert(id, Mass::zero());
        self.requested.insert(id, MassRate::zero());
        self.fulfillment.insert(id, 0.0);
    }

    fn set_negatives_to_zero(&mut self) {
        self.stockpile
            .iter_mut()
            .for_each(|stockpile| {
                stockpile.iter_mut()
                    .for_each(|amount| *amount = amount.max(Mass::zero()));
            });
    }

    fn set_fulfillment(&mut self) {
        self.fulfillment.iter_mut()
            .zip(self.stockpile.iter())
            .zip(self.requested.iter())
            .for_each(|((f, s), r)| {
                f.iter_mut()
                    .zip(s.iter())
                    .zip(r.iter())
                    .for_each(|((f, s), r)| {
                        *f = Self::calculate_fulfillment(*s, *r, INTERVAL);
                    });
            });
    }

    fn calculate_fulfillment(stockpile: Mass, requested: MassRate, interval: DurationFloat) -> f64 {
        (stockpile / interval / requested).min(1.0)
    }

    pub fn decay(&mut self) {
        const YEAR_FRACTION: f64 = System::ResourceDecay.get_interval_as_year_fraction();

        for (component, resource) in self.stockpile.iter_enum_mut() {
            if let Some(annual_decay) = resource.get_annual_decay() {
                let decay = annual_decay.powf(YEAR_FRACTION);
                component
                    .iter_mut()
                    .for_each(|value| {
                        *value *= decay;
                    });
            }
        }
    }
}

#[test]
fn calculate_fulfillment() {
    assert_eq!(1.0, Resources::calculate_fulfillment(Mass::in_kg(1.0), MassRate::in_kg_per_s(1.0), DurationFloat::in_s(1.0)));
    assert_eq!(1.0, Resources::calculate_fulfillment(Mass::in_kg(2.0), MassRate::in_kg_per_s(1.0), DurationFloat::in_s(1.0)));
    assert_eq!(0.5, Resources::calculate_fulfillment(Mass::in_kg(1.0), MassRate::in_kg_per_s(2.0), DurationFloat::in_s(1.0)));
    assert_eq!(0.5, Resources::calculate_fulfillment(Mass::in_kg(1.0), MassRate::in_kg_per_s(1.0), DurationFloat::in_s(2.0)));
}

impl Resources {
    fn reset_requests(&mut self) {
        self.requested.fill_with(MassRate::zero);
    }
}

#[derive(Debug, Default)]
pub struct Production {
    data: FacilityMap<Colony, ProductionUnit>,
}

impl Production {
    pub fn iter(&self) -> Iter<IdMap<Colony, ProductionUnit>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<IdMap<Colony, ProductionUnit>> {
        self.data.iter_mut()
    }

    pub fn iter_enum(&self) -> Zip<Iter<IdMap<Colony, ProductionUnit>>, Iter<Facility>> {
        self.data.iter_enum()
    }

    pub fn iter_enum_mut(&mut self) -> Zip<IterMut<IdMap<Colony, ProductionUnit>>, Iter<Facility>> {
        self.data.iter_enum_mut()
    }

    pub fn kill(&mut self, id: Id<Colony>) {
        self.data
            .iter_mut()
            .for_each(|map| map.kill(id));
    }

    pub fn request_resouces(&mut self, resources: &mut Resources, alloc: &Allocator<Colony>) {
        self.data
            .iter_enum_mut()
            .for_each(|(production, facility)| {
                let production = production.validate(alloc);

                facility.get_inputs()
                    .iter()
                    .for_each(|input| {
                        let requested = resources.requested.get_mut(input.resource);

                        production
                            .iter()
                            .for_each(|(colony, unit)| {
                                let requested = requested.get_mut(colony);
                                *requested += unit.capacity * input.multiplier;
                            });
                    });
            });
    }

    fn get_fulfillment(&mut self, resources: &Resources, alloc: &Allocator<Colony>) {
        self.data
            .iter_enum_mut()
            .for_each(|(production, facility)| {
                let production = &mut production.validate_mut(alloc);

                Self::reset_fulfillment(production);

                facility.get_inputs()
                    .iter()
                    .for_each(|input| {
                        let input_fulfillment = resources.fulfillment.get(input.resource);

                        production
                            .iter_mut()
                            .for_each(|(colony, unit)| {
                                let input_fulfillment = input_fulfillment.get(colony);
                                unit.fulfillment = unit.fulfillment.min(*input_fulfillment);
                            });
                    });
            });
    }

    fn reset_fulfillment(map: &mut Valid<&mut IdMap<Colony, ProductionUnit>>) {
        map.value
            .iter_mut()
            .for_each(|(_, unit)| unit.fulfillment = 1.0);
    }

    fn take_inputs(&mut self, resources: &mut Resources, alloc: &Allocator<Colony>) {
        self.data
            .iter_enum_mut()
            .for_each(|(production, facility)| {
                let mut production = production.validate_mut(alloc);

                for input in facility.get_inputs() {
                    let stockpile = resources.stockpile.get_mut(input.resource);

                    production
                        .iter_mut()
                        .for_each(|(colony, unit)| {
                            let stockpile = stockpile.get_mut(colony);

                            *stockpile -= unit.get_output() * input.multiplier * INTERVAL;
                        });
                }
            });
    }

    fn output(&mut self, resources: &mut Resources, alloc: &Allocator<Colony>) {
        resources.stockpile.iter_mut()
            .zip(self.data.iter_mut())
            .for_each(|(stockpile, production)| {
                let production = production.validate(alloc);

                production
                    .iter()
                    .for_each(|(colony, unit)| {
                        let stockpile = stockpile.get_mut(colony);
                        *stockpile += unit.get_output() * INTERVAL;
                    });
            });
    }
}

#[derive(Debug, Default)]
pub struct ProductionUnit {
    pub capacity: MassRate,
    pub fulfillment: f64,
}

impl ProductionUnit {
    pub fn get_output(&self) -> MassRate {
        self.capacity * self.fulfillment
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Edge<> {
    pub from: Id<Colony>,
    pub to: Id<Colony>,
}

pub struct Shipping {
    pub graph: Graph<Colony, ShippingUnit>,
    pub queue: Component<Colony, Mass>,
}

// impl Shipping {
//     pub fn request_resources(&mut self, resources: &mut Resources, alloc: &Allocator<Colony>) {
//         let graph = self.graph.validate(alloc);
//     }
// }

pub struct ShippingUnit {
    pub flow: MassRate,
    pub fulfillment: f64,
    pub queue: Mass,
}
