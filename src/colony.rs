use crate::body::{Bodies, Body};
use crate::nation::{Nation, Nations};
use crate::systems::System;
use crate::*;
use crate::colony::economy::Economy;

mod food_consumption;
mod food_production;
mod population;
mod economy;

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

    pub economy: Economy,

    pub body: Component<Colony, Id<Body>>,
    pub nation: Component<Colony, Option<Id<Nation>>>,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let food_production = row.food_production_override
            .unwrap_or_else(|| row.population.get_food_requirement());

        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, row.population);

        self.food.insert(id, row.food);
        self.food_production.insert(id, food_production);
        self.hunger_ema.insert(id, ExpMovingAvg::default());

        self.economy.insert(id);

        self.body.insert(id, links.body);
        self.nation.insert(id, Some(links.nation));

        id.id
    }

    pub fn delete(&mut self, id: Id<Colony>) {
        if let Some(id) = self.alloc.validate(id) {
            self.name.get_mut(id).clear();

            self.population.insert(id, Population::zero());

            self.food.insert(id, Mass::zero());
            self.food_production.insert(id, MassRate::zero());
            self.hunger_ema.insert(id, Hunger::default());

            self.nation.insert(id, None);

            let id = id.id;
            self.alloc.kill(id);
        }
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
