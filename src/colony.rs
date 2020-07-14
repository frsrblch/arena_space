use crate::*;
use crate::body::{Bodies, Body};
use crate::nation::{Nations, Nation};
use crate::systems::System;

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

#[derive(Debug, Default)]
pub struct Colonies {
    pub alloc: Allocator<Colony>,

    pub name: Component<Colony, String>,
    pub population: Component<Colony, Population>,

    pub food: Component<Colony, Mass>,
    pub food_production: Component<Colony, MassRate>,
    pub hunger: Component<Colony, f64>,

    pub body: Component<Colony, Id<Body>>,
    pub nation: Component<Colony, Option<Id<Nation>>>,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);
        self.food.insert(id, row.food);

        let food_production = row.food_production_override.unwrap_or(row.population.get_food_requirement());
        self.food_production.insert(id, food_production);

        self.hunger.insert(id, 0.0);

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
            self.alloc.validate(id)
                .map(|id| self.population.get(id))
        }

        // Logistic function:   dN/dt = r * N
        //                      dN/dt = r_max * (K - N) / K * N
        //
        // wherer:              N = population
        //                      r = growth rate (zero growth = 1.0)
        // where:               K = N_max * r_max / (r_max - 1)
        //                      N_max = ρ_max * surface area * land fraction * habitable fraction
        //                      land fraction = land area / total area
        //                      habitable fraction = habitable area / land area
        //                      ρ_max = 12 billion / 104 million sq km
        //
        //                      land usage: https://ourworldindata.org/land-use
        pub fn update_population(&mut self, bodies: &Bodies) {
            let year_fraction = System::ColonyPopulation.get_interval_as_year_fraction();

            self.population.iter_mut()
                .zip(self.hunger.iter())
                .zip(self.body.iter())
                .for_each(|((pop, hunger), body)| {
                    let area = bodies.get_land_area(body);
                    let max_pop = area * Self::MAX_POPULATION_DENSITY;
                    let k = max_pop * (Self::BASE_GROWTH_MULTIPLIER / Self::BASE_GROWTH_RATE);

                    let annual_growth_rate = Self::BASE_GROWTH_MULTIPLIER * (k - *pop) / k * (1.0 - 0.5 * hunger);
                    dbg!(annual_growth_rate);
                    let population_multiplier = annual_growth_rate.powf(year_fraction);

                    *pop *= population_multiplier;
                });
        }

        const BASE_GROWTH_RATE: f64 = 0.02;
        const BASE_GROWTH_MULTIPLIER: f64 = 1.0 + Self::BASE_GROWTH_RATE;

        /// 12 billion / 104e6 sq km
        const MAX_POPULATION_DENSITY: PopulationDensity = PopulationDensity::in_people_per_square_km(12e9 / 104e6);
    }
}

mod production {
    use super::*;

    impl Colonies {
        pub fn update_food_production_rate(&mut self, nation: &Nations, body: &Bodies) {
            let year_fraction = System::ColonyFoodProductionRate.get_interval_as_year_fraction();

            self.food_production.iter_mut()
                .zip(self.population.iter())
                .zip(self.nation.iter())
                .zip(self.body.iter())
                .for_each(|(((production, population), nation_id), body_id)| {

                    let habitability_multiplier = body.properties.get(body_id)
                        .get_habitability()
                        .get_food_production_multiplier();

                    let target_multiplier = nation.get_food_production_target(nation_id)
                        .map(|t| t.get_multiplier())
                        .unwrap_or(0.0);

                    *production += population.get_food_requirement() * year_fraction * target_multiplier * habitability_multiplier;
                });
        }
    }
}

mod food {
    use super::*;

    impl Colonies {
        pub fn get_food(&self, id: Id<Colony>) -> Option<&Mass> {
            self.alloc.validate(id)
                .map(|id| self.food.get(id))
        }

        pub fn produce_and_consume_food(&mut self) {
            self.food.iter_mut()
                .zip(self.hunger.iter_mut())
                .zip(self.food_production.iter())
                .zip(self.population.iter())
                .for_each(|(((food, hunger), production_rate), pop)| {
                    let interval = System::ColonyFoodProduction.get_interval_float();

                    let production = production_rate * interval;

                    let consumption_rate = pop.get_food_requirement();
                    let consumption = consumption_rate * interval;

                    *food += production - consumption;
                    *hunger = -(food.min(Mass::zero()) / consumption);
                    *food = food.max(Mass::zero());
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyLinks;

    #[test]
    fn food_consumption_hungry() {
        let mut time = TimeFloat::default();
        let (mut colony, id) = get_hungry_colony();

        while time < TimeFloat::in_days(10.0) {
            time += System::ColonyFoodProduction.get_interval_float();

            colony.produce_and_consume_food();
        }

        assert_eq!(Mass::zero(), *colony.get_food(id).unwrap());
    }

    #[test]
    fn food_consumption_fed() {
        let mut time = TimeFloat::default();
        let (mut colony, id) = get_fed_colony();

        let starting_food = *colony.get_food(id).unwrap();

        while time < TimeFloat::in_days(100.0) {
            time += System::ColonyFoodProduction.get_interval_float();

            colony.produce_and_consume_food();
        }

        let ending_food = *colony.get_food(id).unwrap();

        assert!(ending_food > starting_food);
    }

    fn get_hungry_colony() -> (Colonies, Id<Colony>) {
        let mut colony = Colonies::default();

        let id = colony.create(
            Colony {
                name: "New Spaceville".to_string(),
                population: Population::in_millions(1.0),
                food: Mass::in_kg(10e6),
                food_production_override: Some(MassRate::zero()),
            },
            ColonyLinks {
                body: body(),
                nation: govt()
            }
        );

        (colony, id)
    }

    fn get_fed_colony() -> (Colonies, Id<Colony>) {
        let (mut colony, id) = get_hungry_colony();

        let id = colony.alloc.validate(id).unwrap();
        let pop = colony.population.get(id);
        let production = colony.food_production.get_mut(id);
        *production = pop.get_food_requirement() * 1.2;

        let id = id.id;
        (colony, id)
    }

    fn body() -> Id<Body> {
        Allocator::<Body>::default().create()
    }

    fn govt() -> Id<Nation> {
        Allocator::<Nation>::default().create().id()
    }

    #[test]
    fn population_growth_fed_colony() {
        let (mut state, id) = get_fed_colony_system_state();
        let colony = &mut state.state.colony;

        let population_before = *colony.get_population(id).unwrap();

        let end_time = state.state.time.get_time() + chrono::Duration::days(30);
        state.update(end_time);

        let colony = &mut state.state.colony;
        let population_after = *colony.get_population(id).unwrap();

        assert!(population_after > population_before);
    }

    fn get_fed_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut state, body, nation) = get_base();

        let population = Population::in_millions(8_532.0);
        let colony = Colony {
            name: "Earth Sphere".to_string(),
            population,
            food: population.get_food_requirement() * DurationFloat::in_days(90.0),
            food_production_override: None
        };
        let colony = state.state.colony.create(colony, ColonyLinks { body, nation });

        (state, colony)
    }

    #[test]
    fn population_growth_hungry_colony() {
        let (mut state, id) = get_hungry_colony_system_state();
        let colony = &mut state.state.colony;

        let population_before = *colony.get_population(id).unwrap();

        let end_time = state.state.time.get_time() + chrono::Duration::days(365);
        state.update(end_time);

        let colony = &mut state.state.colony;
        let population_after = *colony.get_population(id).unwrap();

        assert!(population_after < population_before);
    }

    fn get_hungry_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut state, body, nation) = get_base();

        let population = Population::in_millions(8_532.0);
        let colony = Colony {
            name: "Earth Sphere".to_string(),
            population,
            food: Mass::zero(),
            food_production_override: Some(MassRate::zero()),
        };
        let colony = state.state.colony.create(colony, ColonyLinks { body, nation });

        (state, colony)
    }

    #[test]
    fn population_growth_overpopulated_colony() {
        let (mut state, id) = get_overpopulated_colony_system_state();
        let colony = &mut state.state.colony;

        let population_before = *colony.get_population(id).unwrap();

        let end_time = state.state.time.get_time() + chrono::Duration::days(365);
        state.update(end_time);

        let colony = &mut state.state.colony;
        let population_after = *colony.get_population(id).unwrap();

        assert!(population_after < population_before);
    }

    fn get_overpopulated_colony_system_state() -> (SystemState, Id<Colony>) {
        let (mut state, body, nation) = get_base();

        let population = Population::in_millions(50_000.0);
        let colony = Colony {
            name: "Sardine Can".to_string(),
            population,
            food: population.get_food_requirement() * DurationFloat::in_days(90.0),
            food_production_override: None,
        };
        let colony = state.state.colony.create(colony, ColonyLinks { body, nation });

        (state, colony)
    }

    fn get_base() -> (SystemState, Id<Body>, Id<Nation>) {
        let mut state = SystemState::default();

        let star = crate::star::examples::sol();
        let star = state.state.star.create(star);

        let body = crate::body::examples::earth();
        let body = state.state.body.create(body, BodyLinks { star, parent: None });

        let nation = crate::nation::examples::humanity();
        let nation = state.state.nation.create(nation);

        (state, body, nation)
    }
}