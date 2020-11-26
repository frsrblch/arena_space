use crate::colony::{Colonies, Colony};
use crate::components::*;
use crate::systems::System;
use gen_id::*;
use iter_context::{ContextualIterator, Iter, IterMut, Zip};

// TODO split economy into production, pricing, decay?

const INTERVAL: DurationFloat = crate::systems::System::ColonyProductionCycle.get_interval_float();

impl Colonies {
    pub fn production_cycle(&mut self) {
        self.resources.reset_supply_and_demand();

        self.request_resources();
        self.resources.calculate_fulfillment();
        self.read_fulfillment();
        self.take_inputs();

        self.production.output(&mut self.resources);

        self.resources.add_shipping_flow_to_supply_and_demand();
        self.resources.set_prices();

        // update production rate
    }

    fn request_resources(&mut self) {
        self.production.request_resources(&mut self.resources);
        self.people.request_food(&mut self.resources);
    }

    fn read_fulfillment(&mut self) {
        self.production.get_fulfillment(&self.resources);
    }

    fn take_inputs(&mut self) {
        self.production.take_inputs(&mut self.resources);
        self.people.take_food(&mut self.resources);

        self.resources.set_negatives_to_zero();
    }
}

/// demand = requested (+ shipping out)
/// supply = production (+ shipping in)
#[derive(Debug, Default)]
pub struct Resources {
    pub stockpile: ResourceComponent<Colony, Mass>,
    pub fulfillment: ResourceComponent<Colony, f64>,

    pub supply: ResourceComponent<Colony, MassRate>,
    pub demand: ResourceComponent<Colony, MassRate>,

    pub price: ResourceComponent<Colony, Price>,
    pub price_multiplier: ResourceComponent<Colony, f64>,

    pub shipping: ResourceComponent<Colony, Mass>,
    pub avg_shipping: ResourceComponent<Colony, ExpMovingAvg<MassRate, 12.0>>,
}

impl Resources {
    pub fn insert<I: ValidId<Colony>>(&mut self, id: I) {
        self.stockpile.insert(id, Mass::zero());
        self.fulfillment.insert(id, 0.0);

        self.supply.insert(id, MassRate::zero());
        self.demand.insert(id, MassRate::zero());

        self.price.insert_default_prices(id);
        self.price_multiplier.insert(id, 1.0);

        self.shipping.insert_default(id);
        self.avg_shipping
            .insert(id, ExpMovingAvg::new(MassRate::zero()));
    }

    pub fn print_colony<I: ValidId<Colony>>(&self, id: I) {
        println!("  Stockpile:");

        for ((((stockpile, resource), price), supply), demand) in self
            .stockpile
            .iter_enum()
            .zip(self.price.iter())
            .zip(self.supply.iter())
            .zip(self.demand.iter())
        {
            let amount = stockpile.get(id);
            let price = price.get(id);
            let supply = supply.get(id);
            let demand = demand.get(id);

            if MassRate::zero().ne(supply) || MassRate::zero().ne(demand) {
                println!(
                    "    {}: {}\t{}\tS-D: {:.2}-{:.2}",
                    resource,
                    amount.tons(),
                    price,
                    supply.value,
                    demand.value,
                );
            }
        }
    }

    fn reset_supply_and_demand(&mut self) {
        self.supply.fill_with(MassRate::zero);
        self.demand.fill_with(MassRate::zero);
    }

    // TODO incorporate partial shipping amount so that changes affect prices before the average is updated
    fn add_shipping_flow_to_supply_and_demand(&mut self) {
        let supply = self.supply.iter_mut();
        let demand = self.demand.iter_mut();
        let shipping = self.avg_shipping.iter();

        for ((supply, demand), shipping) in supply.zip(demand).zip(shipping) {
            let supply = supply.iter_mut();
            let demand = demand.iter_mut();
            let shipping = shipping.iter();

            for ((supply, demand), shipping) in supply.zip(demand).zip(shipping) {
                let shipping = shipping.value();
                *supply += shipping.max(MassRate::zero());
                *demand += (-shipping).max(MassRate::zero());
            }
        }
    }

    fn calculate_fulfillment(&mut self) {
        self.fulfillment
            .iter_mut()
            .zip(self.stockpile.iter())
            .zip(self.demand.iter())
            .for_each(|((f, s), r)| {
                f.iter_mut()
                    .zip(s.iter())
                    .zip(r.iter())
                    .for_each(|((f, s), r)| {
                        *f = Self::calculate_fulfillment_inner(*s, *r, INTERVAL);
                    });
            });
    }

    fn calculate_fulfillment_inner(
        stockpile: Mass,
        requested: MassRate,
        interval: DurationFloat,
    ) -> f64 {
        let flow = stockpile / interval;
        let fulfillment = flow / requested;
        fulfillment.min(1.0)
    }

    fn set_negatives_to_zero(&mut self) {
        for stockpile in self.stockpile.iter_mut() {
            for amount in stockpile.iter_mut() {
                *amount = amount.max(Mass::zero());
            }
        }
    }

    fn set_prices(&mut self) {
        let prices = self.price.iter_mut();
        let multiplier = self.price_multiplier.iter_mut();

        let iter = prices
            .zip(multiplier)
            .zip(self.supply.iter())
            .zip(self.demand.iter())
            .zip(crate::PRICE_DEFAULT.iter());

        for ((((prices, multiplier), supply), demand), default) in iter {
            let iter = prices.zip(multiplier).zip(supply).zip(demand);

            for (((price, mult), supply), demand) in iter {
                let ratio = DemandSupplyRatio::calculate(*demand, *supply);
                *price = ratio * *mult * default;
                *mult *= ratio.powf(0.02);
            }
        }
    }

    pub fn decay(&mut self) {
        let year_fraction = System::ResourceDecay.get_interval_as_year_fraction();

        for (component, resource) in self.stockpile.iter_enum_mut() {
            if let Some(annual_decay) = resource.get_annual_decay() {
                let decay = annual_decay.powf(year_fraction);

                component.for_each(|value| *value *= decay);
            }
        }
    }

    pub fn update_shipping_avg(&mut self) {
        const INTERVAL: DurationFloat = System::ShippingAverage.get_interval_float();

        for (shipped, average) in self.shipping.iter_mut().zip(self.avg_shipping.iter_mut()) {
            for (shipped, average) in shipped.iter_mut().zip(average.iter_mut()) {
                average.add_next(*shipped / INTERVAL);
                *shipped = Mass::zero();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct DemandSupplyRatio(f64);

impl DemandSupplyRatio {
    fn calculate(demand: MassRate, supply: MassRate) -> f64 {
        debug_assert!(demand.value >= 0.0);
        debug_assert!(supply.value >= 0.0);

        let value = if supply == demand {
            1.0
        } else if supply == MassRate::zero() {
            DemandSupplyRatio::MAX_VALUE
        } else {
            (demand / supply).min(Self::MAX_VALUE)
        };

        value
    }

    const MAX_VALUE: f64 = 4.0;
}

#[test]
fn demand_supply_ratio_tests() {
    let demand_supply_expected = |demand: f64, supply: f64, expected: f64| {
        let supply = MassRate::in_kg_per_s(supply);
        let demand = MassRate::in_kg_per_s(demand);
        assert_eq!(expected, DemandSupplyRatio::calculate(demand, supply))
    };

    demand_supply_expected(1.0, 1.0, 1.0);
    demand_supply_expected(0.0, 0.0, 1.0);
    demand_supply_expected(1.0, 0.0, DemandSupplyRatio::MAX_VALUE);
    demand_supply_expected(0.0, 1.0, 0.0);
    demand_supply_expected(2.0, 1.0, 2.0);
    demand_supply_expected(1.0, 2.0, 0.5);
    demand_supply_expected(1000.0, 1.0, DemandSupplyRatio::MAX_VALUE);
}

#[derive(Debug, Default)]
pub struct Production {
    data: FacilityMap<Colony, ProductionUnit>,
}

impl Production {
    pub fn print_colony<I: ValidId<Colony>>(&self, id: I) {
        println!("  Production:");

        for (map, facility) in self.data.iter_enum() {
            if let Some(unit) = map.get(id) {
                println!("    {}: {}", facility, unit.get_output().tons_per_day());
            }
        }
    }

    pub fn get(&self, facility: Facility) -> &IdMap<Colony, ProductionUnit> {
        self.data.get(facility)
    }

    pub fn get_mut(&mut self, facility: Facility) -> &mut IdMap<Colony, ProductionUnit> {
        self.data.get_mut(facility)
    }

    pub fn iter(&self) -> Iter<Facility, IdMap<Colony, ProductionUnit>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Facility, IdMap<Colony, ProductionUnit>> {
        self.data.iter_mut()
    }

    pub fn iter_enum(
        &self,
    ) -> Zip<Facility, Iter<Facility, IdMap<Colony, ProductionUnit>>, Iter<Facility, Facility>>
    {
        self.data.iter_enum()
    }

    pub fn iter_enum_mut(
        &mut self,
    ) -> Zip<Facility, IterMut<Facility, IdMap<Colony, ProductionUnit>>, Iter<Facility, Facility>>
    {
        self.data.iter_enum_mut()
    }

    pub fn kill(&mut self, id: Id<Colony>) {
        self.data.iter_mut().for_each(|map| map.kill(id));
    }

    pub fn request_resources(&mut self, resources: &mut Resources) {
        for (production, facility) in self.iter_enum_mut() {
            for input in facility.get_inputs() {
                let demand = resources.demand.get_mut(input.resource);

                for (colony, unit) in production.iter() {
                    let demand = demand.get_mut(colony);
                    let amount = unit.capacity * input.multiplier;
                    *demand += amount;
                }
            }
        }
    }

    fn get_fulfillment(&mut self, resources: &Resources) {
        for (production, facility) in self.iter_enum_mut() {
            Self::reset_fulfillment(production);

            for input in facility.get_inputs() {
                let input_fulfillment = resources.fulfillment.get(input.resource);

                for (colony, unit) in production.iter_mut() {
                    let input_fulfillment = input_fulfillment.get(colony);
                    unit.fulfillment = unit.fulfillment.min(*input_fulfillment);
                }
            }
        }
    }

    fn reset_fulfillment(map: &mut IdMap<Colony, ProductionUnit>) {
        for (_, unit) in map.iter_mut() {
            unit.fulfillment = 1.0;
        }
    }

    fn take_inputs(&mut self, resources: &mut Resources) {
        for (production, facility) in self.iter_enum_mut() {
            for input in facility.get_inputs() {
                let stockpile = resources.stockpile.get_mut(input.resource);

                for (colony, unit) in production.iter() {
                    let stockpile = stockpile.get_mut(colony);
                    *stockpile -= unit.get_output() * input.multiplier * INTERVAL;
                }
            }
        }
    }

    fn output(&mut self, resources: &mut Resources) {
        for (production, facility) in self.iter_enum_mut() {
            let output = facility.get_output();
            let stockpile = resources.stockpile.get_mut(output);
            let supply = resources.supply.get_mut(output);

            for (colony, unit) in production.iter() {
                let output = unit.get_output();

                let stockpile = stockpile.get_mut(colony);
                *stockpile += output * INTERVAL;

                let supply = supply.get_mut(colony);
                *supply += output;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct ProductionUnit {
    pub capacity: MassRate,
    pub fulfillment: f64,
}

impl ProductionUnit {
    pub fn new(capacity: MassRate) -> Self {
        Self {
            capacity,
            fulfillment: 0.0,
        }
    }

    pub fn get_output(&self) -> MassRate {
        self.capacity * self.fulfillment
    }
}

pub struct Shipping {
    pub graph: Graph<Colony, ShippingUnit>,
}

pub struct ShippingUnit {
    pub flow: MassRate,
    pub fulfillment: f64,
    pub queue: Mass,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fulfillment() {
        assert_eq!(
            1.0,
            Resources::calculate_fulfillment_inner(
                Mass::in_kg(1.0),
                MassRate::in_kg_per_s(1.0),
                DurationFloat::in_s(1.0)
            )
        );
        assert_eq!(
            1.0,
            Resources::calculate_fulfillment_inner(
                Mass::in_kg(2.0),
                MassRate::in_kg_per_s(1.0),
                DurationFloat::in_s(1.0)
            )
        );
        assert_eq!(
            0.5,
            Resources::calculate_fulfillment_inner(
                Mass::in_kg(1.0),
                MassRate::in_kg_per_s(2.0),
                DurationFloat::in_s(1.0)
            )
        );
        assert_eq!(
            0.5,
            Resources::calculate_fulfillment_inner(
                Mass::in_kg(1.0),
                MassRate::in_kg_per_s(1.0),
                DurationFloat::in_s(2.0)
            )
        );
    }
}
