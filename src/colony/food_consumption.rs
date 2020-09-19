use super::*;

impl Colonies {
    pub fn get_food(&self, id: Id<Colony>) -> Option<&Mass> {
        self.alloc.validate(id).map(|id| self.food.get(id))
    }

    pub fn produce_and_consume_food(&mut self) {
        const INTERVAL: DurationFloat = System::ColonyFoodProduction.get_interval_float();

        self.food.iter_mut()
            .zip(self.hunger_ema.iter_mut())
            .zip(self.food_production.iter())
            .zip(self.population.iter())
            .for_each(|(((food, hunger_ema), production_rate), population)| {
                let production = production_rate * INTERVAL;
                *food += production;

                let consumption_rate = population.get_food_requirement();
                let consumption = consumption_rate * INTERVAL;
                let consumed = food.request(consumption);

                let hunger_value = 1.0 - consumed / consumption;
                hunger_ema.add_next(hunger_value);
            });
    }

    pub fn food_decay(&mut self) {
        let year_fraction = System::ColonyFoodDecay.get_interval_as_year_fraction();
        let multiplier = Self::ANNUAL_FOOD_DECAY.powf(year_fraction);

        self.food.iter_mut().for_each(|food| *food *= multiplier);
    }

    const ANNUAL_FOOD_DECAY: f64 = 0.925; // seems to maintain a reserve of 4 months
}
