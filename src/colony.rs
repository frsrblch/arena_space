use crate::body::{Bodies, Body};
use crate::nation::{Nation, Nations};
use crate::systems::System;
use crate::*;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,
    pub population: Population,
    pub food: Mass,
    pub food_production_override: Option<MassRate>,
}

dynamic_arena!(Colony);

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub nation: Id<Nation>,
}

type Hunger = ExpMovingAvg<f64, 15.0>;

#[derive(Debug, Default)]
pub struct Colonies {
    pub alloc: Allocator<Colony>,

    pub name: Component<Colony, String>,
    pub population: Component<Colony, Population>,

    pub food: Component<Colony, Mass>,
    pub food_production: Component<Colony, MassRate>,
    pub hunger_ema: Component<Colony, Hunger>,

    pub body: Component<Colony, Id<Body>>,
    pub nation: Component<Colony, Option<Id<Nation>>>,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let id = self.alloc.create();

        let food_production = row.food_production_override
            .unwrap_or_else(|| row.population.get_food_requirement());

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);
        self.food.insert(id, row.food);

        self.food_production.insert(id, food_production);

        self.hunger_ema.insert(id, ExpMovingAvg::default());

        self.body.insert(id, links.body);
        self.nation.insert(id, Some(links.nation));

        id.id
    }

    pub fn delete(&mut self, id: Id<Colony>) {
        if let Some(id) = self.alloc.validate(id) {
            self.population.insert(id, Population::zero());
            self.food.insert(id, Mass::zero());
            self.food_production.insert(id, MassRate::zero());

            self.nation.insert(id, None);

            let id = id.id;
            self.alloc.kill(id);
        }
    }
}

mod population {
    use super::*;

    impl Colonies {
        pub fn get_population(&self, id: Id<Colony>) -> Option<&Population> {
            self.alloc.validate(id).map(|id| self.population.get(id))
        }

        // Logistic function:   dN/dt = r * N
        //                      dN/dt = r_max * (K - N) / K * N
        //
        // where:               N = population
        //                      r = growth rate (zero growth = 1.0)
        // where:               K = N_max * r_max / (r_max - 1)
        //                      N_max = ρ_max * surface area * land fraction * habitable fraction
        //                      land fraction = land area / total area
        //                      habitable fraction = habitable area / land area
        //                      ρ_max = 12 billion / 104 million sq km
        //
        //                      land usage: https://ourworldindata.org/land-use
        pub fn update_population(&mut self, bodies: &mut Bodies) {
            bodies.sum_population(self);

            self.population.iter_mut()
                .zip(self.hunger_ema.iter())
                .zip(self.body.iter())
                .for_each(|((pop, hunger), body)| {
                    *pop *= Self::get_population_multiplier(pop, hunger, body, bodies)
                });
        }

        fn get_population_multiplier(
            pop: &Population,
            hunger: &Hunger,
            body: &Id<Body>,
            bodies: &Bodies,
        ) -> f64 {
            let area = bodies.get_land_area(body);
            let max_pop = area * MAX_POPULATION_DENSITY;
            let k = max_pop * (BASE_GROWTH_MULTIPLIER / BASE_GROWTH_RATE);

            let body_population = bodies.population.get(body).unwrap_or(pop);
            let mut k_factor = (k - *body_population) / k;
            k_factor = k_factor.max(0.01);

            let hunger_multiplier = 1.0 - hunger.value();

            let annual_growth_rate = BASE_GROWTH_MULTIPLIER * k_factor * hunger_multiplier;

            annual_growth_rate.powf(YEAR_FRACTION)
        }

        // fn get_k_factor(body: &Id<Body>, bodies: &Bodies)
    }

    const YEAR_FRACTION: f64 = System::ColonyPopulation.get_interval_as_year_fraction();
    const BASE_GROWTH_RATE: f64 = 0.025;
    const BASE_GROWTH_MULTIPLIER: f64 = 1.0 + BASE_GROWTH_RATE;

    /// 12 billion / 104e6 sq km
    const MAX_POPULATION_DENSITY: PopulationDensity =
        PopulationDensity::in_people_per_square_km(12e9 / 104e6);
}

mod production {
    use super::*;
    use crate::body::Habitability;
    use crate::nation::FoodProductionTarget;

    impl Colonies {
        pub fn update_food_production_rate(&mut self, nations: &Nations, bodies: &Bodies) {
            self.food_production
                .iter_mut()
                .zip(self.population.iter())
                .zip(self.nation.iter())
                .zip(self.body.iter())
                .for_each(|(((production, population), nation_id), body_id)| {
                    *production += Self::get_new_food_production(
                        production, population, nation_id, body_id, nations, bodies,
                    );
                });
        }

        fn get_new_food_production(
            production: &MassRate,
            population: &Population,
            nation_id: &Option<Id<Nation>>,
            body_id: &Id<Body>,
            nations: &Nations,
            bodies: &Bodies,
        ) -> MassRate {
            let consumption = population.get_food_requirement();

            let habitability = bodies.get_habitability(body_id);

            let national_target = *nations
                .get_food_production_target(nation_id)
                .unwrap_or(&FoodProductionTarget::Stable);

            let target = Self::get_food_production_target(
                national_target,
                production,
                consumption,
                habitability,
            );

            let production_multiplier = Self::get_production_rate_multiplier(habitability, target);

            consumption * YEAR_FRACTION * production_multiplier
        }

        fn get_food_production_target(
            national_target: FoodProductionTarget,
            production: &MassRate,
            consumption: MassRate,
            habitability: Habitability,
        ) -> FoodProductionTarget {
            let self_sufficiency = production / consumption;

            // expand production if colony is not self-sufficient and is well-suited to do so
            if self_sufficiency < 1.02 && habitability == Habitability::Optimal {
                FoodProductionTarget::Expand
            } else {
                national_target
            }
        }

        fn get_production_rate_multiplier(
            habitability: Habitability,
            target: FoodProductionTarget,
        ) -> f64 {
            let habitability_multiplier = match target {
                FoodProductionTarget::Stable => 0.0,
                FoodProductionTarget::Expand => {
                    habitability.get_food_production_expansion_multiplier()
                }
                FoodProductionTarget::Contract => {
                    habitability.get_food_production_contraction_multiplier()
                }
            };

            let target_multiplier = target.get_multiplier();

            habitability_multiplier * target_multiplier
        }
    }

    const YEAR_FRACTION: f64 = System::ColonyFoodProductionRate.get_interval_as_year_fraction();
}

mod food {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyLinks;

    #[test]
    fn population_growth_fed_colony() {
        let (mut system_state, id) = get_fed_colony_system_state();
        let colonies = &system_state.state.colony;

        let population_before = *colonies.get_population(id).unwrap();

        let end_time = system_state.state.time.get_time() + chrono::Duration::days(365);
        system_state.update(end_time);

        let colonies = &mut system_state.state.colony;
        let population_after = *colonies.get_population(id).unwrap();

        assert!(population_after > population_before);
    }

    #[test]
    fn population_growth_fed_colony_over_time() {
        let (mut system_state, id) = get_fed_colony_system_state();
        let colonies = &system_state.state.colony;

        let population_before = *colonies.get_population(id).unwrap();

        let end_time = system_state.state.time.get_time() + chrono::Duration::days(365 * 10);
        system_state.update(end_time);

        let colonies = &mut system_state.state.colony;
        let population_after = *colonies.get_population(id).unwrap();

        assert!(population_after > population_before);
    }

    fn get_fed_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut system_state, body, nation) = get_base();
        let colonies = &mut system_state.state.colony;

        let population = Population::in_millions(8_532.0);
        let colony = Colony {
            name: "Earth Sphere".to_string(),
            population,
            food: population.get_food_requirement() * DurationFloat::in_days(90.0),
            food_production_override: None,
        };
        let colony = colonies.create(colony, ColonyLinks { body, nation });

        (system_state, colony)
    }

    #[test]
    fn population_growth_hungry_colony() {
        let (mut system_state, id) = get_hungry_colony_system_state();
        let colonies = &system_state.state.colony;

        let population_before = *colonies.get_population(id).unwrap();

        let end_time = system_state.state.time.get_time() + chrono::Duration::days(365 * 10);
        system_state.update(end_time);

        let colonies = &mut system_state.state.colony;
        let population_after = *colonies.get_population(id).unwrap();

        assert!(population_after < population_before);
    }

    fn get_hungry_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut system_state, body, nation) = get_base();
        let colonies = &mut system_state.state.colony;

        let population = Population::in_millions(8_532.0);
        let colony = Colony {
            name: "Earth Sphere".to_string(),
            population,
            food: Mass::zero(),
            food_production_override: Some(population.get_food_requirement() * 0.2),
        };
        let colony = colonies.create(colony, ColonyLinks { body, nation });

        (system_state, colony)
    }

    #[test]
    fn population_growth_overpopulated_colony() {
        let (mut system_state, id) = get_overpopulated_colony_system_state();
        let colonies = &system_state.state.colony;

        let population_before = *colonies.get_population(id).unwrap();

        let end_time = system_state.state.time.get_time() + chrono::Duration::days(365);
        system_state.update(end_time);

        let colonies = &mut system_state.state.colony;
        let population_after = *colonies.get_population(id).unwrap();

        assert!(population_after < population_before);
    }

    fn get_overpopulated_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut system_state, body, nation) = get_base();
        let colonies = &mut system_state.state.colony;

        let population = Population::in_millions(500_000.0);
        let colony = Colony {
            name: "Sardine Can".to_string(),
            population,
            food: population.get_food_requirement() * DurationFloat::in_days(90.0),
            food_production_override: None,
        };
        let colony = colonies.create(colony, ColonyLinks { body, nation });

        (system_state, colony)
    }

    fn get_base() -> (SystemState, Id<Body>, Id<Nation>) {
        let mut system_state = SystemState::default();
        let nations = &mut system_state.state.nation;

        let star = crate::star::examples::sol();
        let star = system_state.state.star.create(star);

        let body = crate::body::examples::earth();
        let body = system_state.state.body.create(
            body,
            BodyLinks { star, parent: None }
        );

        let nation = crate::nation::examples::humanity();
        let nation = nations.create(nation);

        (system_state, body, nation)
    }
}
