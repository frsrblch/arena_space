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

    pub fn get_food_production_target(&self, id: impl TryIndexes<Nation>) -> Option<&FoodProductionTarget> {
        id.id()
            .and_then(|id| self.alloc.validate(id))
            .map(|id| self.agriculture.get(id))
    }
}

mod population {
    use super::*;

    impl Nations {
        pub(super) fn sum_population(&mut self, colonies: &Colonies) {
            self.zero_population();
            self.add_population_from_colonies(colonies);
        }

        fn zero_population(&mut self) {
            self.population
                .iter_mut()
                .for_each(|v| *v = Population::zero());
        }

        fn add_population_from_colonies(&mut self, colony: &Colonies) {
            colony.population.iter()
                .zip(colony.nation.iter())
                .for_each(|(population, nation)| {
                    self.add_population(population, nation);
                });
        }

        fn add_population(&mut self, population: &Population, nation: &Option<Id<Nation>>) {
            if let Some(nation) = self.alloc.validate(nation) {
                let national_population = self.population.get_mut(nation);
                *national_population += population;
            }
        }
    }
}

mod food {
    use super::*;

    impl Nations {
        pub fn update_food_targets(&mut self, colony: &Colonies) {
            self.sum_population(colony);
            self.sum_food_production(colony);
            self.set_food_production_target();
        }

        fn sum_food_production(&mut self, colony: &Colonies) {
            self.zero_food_production();
            self.add_production_from_colonies(colony);
        }

        fn zero_food_production(&mut self) {
            self.food_production
                .iter_mut()
                .for_each(|v| *v = MassRate::zero());
        }

        fn add_production_from_colonies(&mut self, colony: &Colonies) {
            colony.food_production.iter()
                .zip(colony.nation.iter())
                .for_each(|(production, govt)| {
                    if let Some(govt) = self.alloc.validate(*govt) {
                        let govt_production = self.food_production.get_mut(govt);
                        *govt_production += production;
                    }
                });
        }

        fn set_food_production_target(&mut self) {
            self.agriculture.iter_mut()
                .zip(self.food_production.iter())
                .zip(self.population.iter())
                .for_each(|((agri, food_production), pop)| {
                    *agri = Self::determine_food_production_target(*food_production, *pop);
                });
        }

        fn determine_food_production_target(food_production: MassRate, population: Population) -> FoodProductionTarget {
            let food_demand = population.get_food_requirement();

            match food_production / food_demand {
                ratio if ratio > 1.1 => FoodProductionTarget::Contract,
                ratio if ratio < 1.02 => FoodProductionTarget::Expand,
                _ => FoodProductionTarget::Stable,
            }
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
    use crate::colony::{Colony, ColonyLinks};
    use crate::body::BodyLinks;
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
            BodyLinks { star, parent: None }
        );

        let moon = state.state.body.create(
            crate::body::examples::luna(),
            BodyLinks { star, parent: Some(earth) }
        );

        let nation = &mut state.state.nation;
        let colony = &mut state.state.colony;

        let nation_id = nation.create(Nation { name: "Nation".to_string() });

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
                nation: nation_id
            }
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
                nation: nation_id
            }
        );

        if let Some(fed) = colony.alloc.validate(fed) {
            colony.food_production.insert(fed, population.get_food_requirement());
        }

        (state, nation_id)
    }
}

pub mod examples {
    use super::Nation;

    pub fn humanity() -> Nation {
        Nation {
            name: "Humanity".to_string()
        }
    }
}