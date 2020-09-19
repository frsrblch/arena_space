use super::*;
use crate::colony::Colonies;

#[derive(Debug, Clone)]
pub struct Nation {
    pub name: String,
}

dynamic_arena!(Nation);

#[derive(Debug, Default)]
pub struct Nations {
    pub alloc: Allocator<Nation>,

    pub name: Component<Nation, String>,

    pub population: Component<Nation, Population>,
    pub food_production: Component<Nation, MassRate>,
    pub agriculture: Component<Nation, FoodProductionTarget>,
}

impl Nations {
    pub fn create(&mut self, row: Nation) -> Id<Nation> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, Population::default());
        self.food_production.insert(id, MassRate::default());
        self.agriculture.insert(id, FoodProductionTarget::Stable);

        id.id
    }

    pub fn get_food_production_target<ID: TryIndexes<Nation>>(&self, id: ID) -> Option<FoodProductionTarget> {
        id.id()
            .and_then(|id| self.alloc.validate(id))
            .map(|id| self.agriculture.get(id))
            .copied()
    }
}

mod food_production_targets {
    use super::*;

    impl Nations {
        pub fn update_food_targets(&mut self, colony: &Colonies) {
            self.sum_population(colony);
            self.sum_food_production(colony);
            self.set_food_production_target();
        }

        fn sum_population(&mut self, colonies: &Colonies) {
            self.population.sum_from(&colonies.population, &colonies.nation, &self.alloc);
        }

        fn sum_food_production(&mut self, colony: &Colonies) {
            self.food_production.sum_from(&colony.food_production, &colony.nation, &self.alloc);
        }

        fn set_food_production_target(&mut self) {
            let food_consumption = self.population.iter()
                .map(Population::get_food_requirement);

            self.agriculture
                .iter_mut()
                .zip(self.food_production.iter())
                .zip(food_consumption)
                .for_each(|((agri, food_production), food_consumption)| {
                    *agri = FoodProductionTarget::new(*food_production, food_consumption);
                });
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FoodProductionTarget {
    Expand,
    Stable,
    Contract,
}

impl FoodProductionTarget {
    pub fn new(food_production: MassRate, consumption: MassRate) -> Self {
        match food_production / consumption {
            ratio if ratio > 1.1 => FoodProductionTarget::Contract,
            ratio if ratio < 1.02 => FoodProductionTarget::Expand,
            _ => FoodProductionTarget::Stable,
        }
    }

    pub fn get_multiplier(&self) -> f64 {
        match self {
            FoodProductionTarget::Expand => 0.2,
            FoodProductionTarget::Stable => 0.0,
            FoodProductionTarget::Contract => -0.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::body::BodyLinks;
    use crate::colony::{Colony, ColonyLinks};
    use crate::star::{Star, StarType};

    #[allow(dead_code)]
    pub fn get_fed_and_hungry_colonies() -> (SystemState, Id<Nation>) {
        let mut state = SystemState::default();

        let star = state.state.star.create(Star {
            name: "Sol".to_string(),
            position: Default::default(),
            star_type: StarType::G(Fraction::new(0.5)),
        });

        let earth = state.state.body.create(
            crate::body::examples::earth(),
            BodyLinks { star, parent: None },
        );

        let moon = state.state.body.create(
            crate::body::examples::luna(),
            BodyLinks { star, parent: Some(earth) }
        );

        let nation = &mut state.state.nation;
        let colony = &mut state.state.colony;

        let nation_id = nation.create(Nation {
            name: "Nation".to_string(),
        });

        let population = Population::in_millions(10.0);
        let five_days_worth = population.get_food_requirement() * DurationFloat::in_days(5.0);

        let _unfed = colony.create(
            Colony {
                name: "Unfed".to_string(),
                population,
                food: five_days_worth,
                food_production_override: Some(MassRate::zero()),
            },
            ColonyLinks {
                body: moon,
                nation: nation_id,
            },
        );

        let fed = colony.create(
            Colony {
                name: "Fed".to_string(),
                population,
                food: five_days_worth,
                food_production_override: None, // defaults to population requirement
            },
            ColonyLinks {
                body: earth,
                nation: nation_id,
            },
        );

        if let Some(fed) = colony.alloc.validate(fed) {
            let food = population.get_food_requirement();
            colony.food_production.insert(fed, food);
        }

        (state, nation_id)
    }

    use FoodProductionTarget::*;
    #[test]
    fn food_production_target_if_well_fed() {
        let food_consumption = MassRate::in_kg_per_s(1.0);
        let food_production = food_consumption * 1.05;

        assert_eq!(Stable, FoodProductionTarget::new(food_production, food_consumption));
    }

    #[test]
    fn food_production_target_if_under_fed() {
        let food_consumption = MassRate::in_kg_per_s(1.0);
        let food_production = food_consumption * 0.5;

        assert_eq!(Expand, FoodProductionTarget::new(food_production, food_consumption));
    }

    #[test]
    fn food_production_target_if_over_fed() {
        let food_consumption = MassRate::in_kg_per_s(1.0);
        let food_production = food_consumption * 1.5;

        assert_eq!(Contract, FoodProductionTarget::new(food_production, food_consumption));
    }
}

pub mod examples {
    use super::Nation;

    pub fn humanity() -> Nation {
        Nation {
            name: "Humanity".to_string(),
        }
    }
}
