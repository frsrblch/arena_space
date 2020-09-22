use super::*;

#[derive(Debug, Default)]
pub struct Economy {
    pub stockpile: ResourceComponent<Colony, Mass>,
    pub requested: ResourceComponent<Colony, Mass>,
    pub fulfillment: ResourceComponent<Colony, f64>,

    pub production: FacilityMap<Colony, MassRate>,
}

impl Economy {
    pub fn insert<I: Indexes<Colony>>(&mut self, id: I) {
        self.stockpile.insert(id, Mass::zero());
        self.requested.insert(id, Mass::zero());
        self.fulfillment.insert(id, 0.0);
    }

    pub fn production_cycle(&mut self, alloc: &Allocator<Colony>) {
        self.requested.fill_with(Mass::zero);

        self.request_materials_from_colonies(alloc);
        self.request_materials_for_shipping();

        self.set_fulfillment_for_requests();

        self.consume_and_produce(alloc);
        self.marshal_materials_for_shipping();
    }

    fn request_materials_from_colonies(&mut self, alloc: &Allocator<Colony>) {
        for (production, facility) in self.production.iter_mut() {
            let map = production.validate(alloc);

            for input in facility.get_inputs() {
                let requested = self.requested.get_mut(input.resource);

                for (colony, production) in map.iter() {
                    let amount_requested = requested.get_mut(colony);
                    *amount_requested += production * input.multiplier * Economy::production_interval();
                }
            }
        }
    }

    fn set_fulfillment_for_requests(&mut self) {
        let iter = self.fulfillment.iter_mut()
            .zip(self.requested.iter())
            .zip(self.stockpile.iter());

        for ((fulfillment, requested), stockpile) in iter {
            let iter = fulfillment.iter_mut()
                .zip(requested.iter())
                .zip(stockpile.iter());

            for ((fulfillment, &requested), &stockpile) in iter {
                *fulfillment = Self::get_fulfillment(requested, stockpile);
            }
        }
    }

    fn get_fulfillment(requested: Mass, stockpile: Mass) -> f64 {
        debug_assert!(requested >= Mass::zero());
        debug_assert!(stockpile >= Mass::zero());

        if stockpile >= requested {
            1.0
        } else {
            stockpile / requested
        }
    }

    fn consume_and_produce(&mut self, alloc: &Allocator<Colony>) {
        for (production, facility) in self.production.iter_mut() {
            let production = production.validate(alloc);

            let min_fulfillment = Self::get_min_fulfillment(&production, &self.fulfillment, *facility);

            Self::consume_inputs(&production, facility, &mut self.stockpile, &min_fulfillment);
            Self::add_output(&production, facility, &mut self.stockpile, &min_fulfillment);
        }
    }

    fn consume_inputs(
        production: &ValidMap<Colony, MassRate>,
        facility: &Facility,
        stockpiles: &mut ResourceComponent<Colony, Mass>,
        min_fulfillment: &Vec<f64>,
    ) {
        for input in facility.get_inputs().iter() {
            let stockpile = stockpiles.get_mut(input.resource);

            production.iter()
                .zip(min_fulfillment)
                .for_each(|((colony, production), min_fulfillment)| {
                    let stockpile = stockpile.get_mut(colony);

                    let consumed = production * input.multiplier * min_fulfillment * Self::production_interval();
                    *stockpile = (*stockpile - consumed).max(Mass::zero());
                });
        }
    }

    fn add_output(
        production: &ValidMap<Colony, MassRate>,
        facility: &Facility,
        stockpiles: &mut ResourceComponent<Colony, Mass>,
        min_fulfillment: &Vec<f64>,
    ) {
        let stockpile = stockpiles.get_mut(facility.get_output());

        production.iter()
            .zip(min_fulfillment)
            .for_each(|((colony, production), min_fulfillment)| {
                let stockpile = stockpile.get_mut(colony);
                *stockpile += min_fulfillment * production * Self::production_interval();
            });
    }

    fn get_min_fulfillment(
        production: &ValidMap<Colony, MassRate>,
        fulfillment: &ResourceComponent<Colony, f64>,
        facility: Facility
    ) -> Vec<f64> {
        let input_fulfillment = Self::get_input_fulfillment(fulfillment, facility);

        let get_min_fulfillment = |colony: ValidRef<Colony>| input_fulfillment.iter()
            .fold(1.0f64, |min, f| f.get(colony).min(min));

        production.iter()
            .map(|(c, _)| c)
            .map(get_min_fulfillment)
            .collect()
    }

    fn get_input_fulfillment(fulfillment: &ResourceComponent<Colony, f64>, facility: Facility) -> Vec<&Component<Colony, f64>> {
        facility.get_inputs()
            .iter()
            .map(|input| fulfillment.get(input.resource))
            .collect()
    }

    fn request_materials_for_shipping(&mut self) {
        // TODO
    }

    fn marshal_materials_for_shipping(&mut self) {
        // TODO
    }

    const fn production_interval() -> DurationFloat {
        System::ColonyProductionCycle.get_interval_float()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fulfillment_when_stockpile_is_larger_than_requested() {
        let requested = Mass::in_kg(1.0);
        let stockpile = Mass::in_kg(10.0);

        let fulfillment = Economy::get_fulfillment(requested, stockpile);

        assert_eq!(1.0, fulfillment);
    }

    #[test]
    fn fulfillment_when_nothing_requested() {
        let requested = Mass::in_kg(0.0);
        let stockpile = Mass::in_kg(10.0);

        let fulfillment = Economy::get_fulfillment(requested, stockpile);

        assert_eq!(1.0, fulfillment);
    }

    #[test]
    fn fulfillment_when_requested_exceeds_stockpile() {
        let requested = Mass::in_kg(10.0);
        let stockpile = Mass::in_kg(2.0);

        let fulfillment = Economy::get_fulfillment(requested, stockpile);

        assert_eq!(0.2, fulfillment);
    }

    #[test]
    fn fulfillment_when_requested_from_empty_stockpile() {
        let requested = Mass::in_kg(10.0);
        let stockpile = Mass::in_kg(0.0);

        let fulfillment = Economy::get_fulfillment(requested, stockpile);

        assert_eq!(0.0, fulfillment);
    }

    fn get_economy() -> (Economy, Allocator<Colony>, Id<Colony>) {
        let mut alloc = Allocator::<Colony>::default();
        let mut econ = Economy::default();

        let colony = alloc.create();
        econ.insert(colony);

        let colony = colony.id;

        (econ, alloc, colony)
    }

    #[test]
    fn production_adds_output_to_stockpile() {
        let (mut economy, alloc, id) = get_economy();
        economy.production.get_mut(Facility::Farmland).insert(id, MassRate::in_kg_per_s(2.0));

        economy.production_cycle(&alloc);

        let id = alloc.validate(id).unwrap();
        let food = *economy.stockpile.get(Resource::Food).get(id);
        assert!(food > Mass::zero());
    }

    #[test]
    fn production_no_output_if_no_inputs() {
        let (mut economy, alloc, id) = get_economy();
        economy.production.get_mut(Facility::Foundry).insert(id, MassRate::in_kg_per_s(2.0));

        economy.production_cycle(&alloc);

        let id = alloc.validate(id).unwrap();
        let metal = *economy.stockpile.get(Resource::Metal).get(id);
        assert_eq!(Mass::zero(), metal);
    }

    #[test]
    fn production_inputs_consumed_by_output() {
        let (mut economy, alloc, id) = get_economy();
        economy.production.get_mut(Facility::Foundry).insert(id, MassRate::in_kg_per_s(2.0));
        let colony = alloc.validate(id).unwrap();
        economy.stockpile.get_mut(Resource::Ore).insert(colony, Mass::in_kg(4.0));

        economy.production_cycle(&alloc);

        let id = alloc.validate(id).unwrap();

        let ore = *economy.stockpile.get(Resource::Ore).get(id);
        assert_eq!(ore, Mass::zero());
    }

    #[test]
    fn production_output_limited_by_input_stockpile() {
        let mut alloc = Allocator::<Colony>::default();
        let mut econ = Economy::default();

        let c1 = alloc.create();
        econ.insert(c1);
        econ.stockpile.get_mut(Resource::Ore).insert(c1, Mass::in_kg(1.0));
        econ.production.get_mut(Facility::Foundry).insert_valid(c1, MassRate::in_kg_per_s(100.0));
        let c1 = c1.id;

        let c2 = alloc.create();
        econ.insert(c2);
        econ.stockpile.get_mut(Resource::Ore).insert(c2, Mass::in_kg(2.0));
        econ.production.get_mut(Facility::Foundry).insert_valid(c2, MassRate::in_kg_per_s(100.0));
        let c2 = c2.id;

        econ.production_cycle(&alloc);

        let c1 = alloc.validate(c1).unwrap();
        let c2 = alloc.validate(c2).unwrap();

        let metal = econ.stockpile.get(Resource::Metal);
        let c1_metal = *metal.get(c1);
        let c2_metal = *metal.get(c2);

        assert!(c2_metal > c1_metal);
    }
}