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

    last_agri_update: TimeFloat,
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
                .for_each(|(pop, govt)| {
                    if let Some(govt) = self.alloc.validate(govt) {
                        let govt_pop = self.population.get_mut(govt);
                        *govt_pop += pop;
                    }
                });
        }
    }
}

mod food {
    use super::*;

    impl Nations {
        pub fn update_food_targets(&mut self, colony: &Colonies) {
            self.sum_population(colony);
            self.sum_food_production(colony);
            self.set_agriculture_directive();
        }

        pub fn sum_food_production(&mut self, colony: &Colonies) {
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

        fn set_agriculture_directive(&mut self) {
            self.agriculture.iter_mut()
                .zip(self.food_production.iter())
                .zip(self.population.iter())
                .for_each(|((agri, food_production), pop)| {
                    let food_demand = pop.get_food_requirement();

                    *agri = match food_production / food_demand {
                        ratio if ratio > 1.1 => FoodProductionTarget::Contract,
                        ratio if ratio < 1.02 => FoodProductionTarget::Expand,
                        _ => FoodProductionTarget::Stable,
                    };
                });
        }

        pub fn get_food_production_target<ID>(&self, id: ID) -> Option<&FoodProductionTarget>
        where
            <Nation as Arena>::Allocator: Validates<ID, Nation>
        {
            self.alloc
                .validate(id)
                .map(|id| self.agriculture.get(id))
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
                food_production: Some(MassRate::zero()),
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
                food_production: None, // defaults to population requirement
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