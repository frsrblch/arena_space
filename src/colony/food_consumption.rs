use super::*;

impl Colonies {
    pub fn produce_and_consume_food(&mut self) {
        const INTERVAL: DurationFloat = System::ColonyFoodProduction.get_interval_float();

        let food_consumption = self.population.iter()
            .map(Population::get_food_requirement);

        self.food.iter_mut()
            .zip(self.hunger_ema.iter_mut())
            .zip(self.food_production.iter())
            .zip(food_consumption)
            .for_each(|(((food, hunger_ema), production_rate), food_consumption)| {
                produce_and_consume_food(food, hunger_ema, *production_rate, food_consumption, INTERVAL);
            });
    }

    pub fn food_decay(&mut self) {
        const ANNUAL_FOOD_DECAY: f64 = 0.925; // seems to maintain a reserve of 4 months
        const YEAR_FRACTION: f64 = System::ColonyFoodDecay.get_interval_as_year_fraction();

        let multiplier = ANNUAL_FOOD_DECAY.powf(YEAR_FRACTION);

        self.food.iter_mut().for_each(|food| *food *= multiplier);
    }
}

fn produce_and_consume_food(food: &mut Mass, hunger_ema: &mut Hunger, food_production: MassRate, food_consumption: MassRate, interval: DurationFloat) {
    let production = food_production * interval;
    *food += production;

    let consumption = food_consumption * interval;
    let consumed = food.request(consumption);

    let hunger_value = 1.0 - consumed / consumption;
    hunger_ema.add_next(hunger_value);
}