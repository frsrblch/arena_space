use crate::*;
use crate::body::{Bodies, Body};
use crate::nation::{Nations, Nation};

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,
    pub population: Population,
    pub food: Mass,
    pub food_production: Option<MassRate>,
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

    last_food_update: TimeFloat,
    last_production_update: TimeFloat,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);
        self.food.insert(id, row.food);

        let food_production = row.food_production.unwrap_or(row.population.get_food_requirement());
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
    }
}

mod production {
    use super::*;

    impl Colonies {
        pub fn update_food_production_rate(&mut self, nation: &Nations, body: &Bodies) {
            let year_fraction = Self::get_year_fraction();

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

        fn get_year_fraction() -> f64 {
            Self::production_update_interval() / DurationFloat::in_s(365.25 * 24.0 * 3600.0)
        }

        fn production_update_interval() -> DurationFloat {
            crate::systems::System::ColonyFoodProductionRate.get_interval().into()
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
                    let interval = Self::food_update_interval();

                    let production = production_rate * interval;

                    let consumption_rate = pop.get_food_requirement();
                    let consumption = consumption_rate * interval;

                    *food += production - consumption;
                    *hunger = -(food.min(Mass::zero()) / consumption);
                    *food = food.max(Mass::zero());
                });
        }

        pub(super) fn food_update_interval() -> DurationFloat {
            crate::systems::System::ColonyFoodProduction.get_interval().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn food_consumption_hungry() {
        let mut time = TimeFloat::default();
        let (mut colony, id) = get_hungry_colony();

        while time < TimeFloat::in_days(10.0) {
            time += Colonies::food_update_interval();

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
            time += Colonies::food_update_interval();

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
                food_production: Some(MassRate::zero()),
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
}