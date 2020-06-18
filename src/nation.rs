use super::*;

#[derive(Debug, Default)]
pub struct Nation {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,

    pub population: Component<Self, Population>,
    pub food_production: Component<Self, MassRate>,
    pub agriculture: Component<Self, FoodProductionTarget>,

    last_agri_update: Time,
}

dynamic_arena!(Nation, u16);

impl Nation {
    pub fn create(&mut self, row: GovernmentRow) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, Population::default());
        self.food_production.insert(id, MassRate::default());
        self.agriculture.insert(id, FoodProductionTarget::Stable);

        id.id
    }
}

#[derive(Debug, Clone)]
pub struct GovernmentRow {
    pub name: String,
}

mod population {
    use super::*;
    use crate::colony::Colony;

    impl Nation {
        pub fn sum_population(&mut self, colonies: &Colony) {
            self.zero_population();
            self.add_population_from_colonies(colonies);
        }

        fn zero_population(&mut self) {
            self.population
                .iter_mut()
                .for_each(|v| *v = Population::zero());
        }

        fn add_population_from_colonies(&mut self, colony: &Colony) {
            colony.population.iter()
                .zip(colony.nation.iter())
                .for_each(|(pop, govt)| {
                    if let Some(govt) = self.alloc.validate(govt) {
                        if let Some(govt_pop) = self.population.get_mut(govt) {
                            *govt_pop += pop;
                        }
                    }
                });
        }
    }
}

mod food {
    use super::*;
    use crate::colony::Colony;

    impl Nation {
        pub fn update_agri_production(&mut self, colony: &Colony, time: Time) {
            if time > self.next_agri_update() {
                self.sum_population(colony);
                self.sum_food_production(colony);
                self.set_agriculture_directive();

                self.last_agri_update += Self::AGRI_UPDATE_INTERVAL;
            }
        }

        fn next_agri_update(&self) -> Time {
            self.last_agri_update + Self::AGRI_UPDATE_INTERVAL
        }

        const AGRI_UPDATE_INTERVAL: Duration = Duration::in_s(30.0 * 3600.0 * 24.0);

        pub fn sum_food_production(&mut self, colony: &Colony) {
            self.zero_food_production();
            self.add_production_from_colonies(colony);
        }

        fn zero_food_production(&mut self) {
            self.food_production
                .iter_mut()
                .for_each(|v| *v = Default::default());
        }

        fn add_production_from_colonies(&mut self, colony: &Colony) {
            colony.food_production.iter()
                .zip(colony.nation.iter())
                .for_each(|(production, govt)| {
                    if let Some(govt) = self.alloc.validate(*govt) {
                        if let Some(govt_production) = self.food_production.get_mut(govt) {
                            *govt_production += production;
                        }
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
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FoodProductionTarget {
    Expand,
    Stable,
    Contract,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colony::{Colony, ColonyRow, ColonyLinks};
    use crate::body::Body;

    #[test]
    fn watch_two_colonies() {
        let mut time = Time::in_days(0.0);
        let (mut govt, mut colony) = get_fed_and_hungry_colonies();

        while time < Time::in_days(10.0) {
            time += Duration::in_days(1.0);

            colony.update_food(time);
            govt.update_agri_production(&colony, time);

            colony.alloc.ids()
                .for_each(|id| {
                    println!(
                        "{:<8}  Food: {:>10}    Population: {:>10}",
                        format!("{}:", colony.name.get(id).unwrap()),
                        format!("{}", colony.food.get(id).unwrap().tons()),
                        format!("{}", colony.population.get(id).unwrap().value),
                    );
                });
            println!();
        }

        // assert!(false);
    }

    fn get_fed_and_hungry_colonies() -> (Nation, Colony) {
        let (mut govt, mut colony) = (Nation::default(), Colony::default());

        let government = govt.create(GovernmentRow { name: "Nation".to_string() });

        let population = Population::in_millions(10.0);
        let five_days_worth = population.get_food_requirement() * Duration::in_days(5.0);

        let _unfed = colony.create(
            ColonyRow {
                name: "Unfed".to_string(),
                population,
                food: five_days_worth,
            },
            ColonyLinks {
                body: get_body(),
                nation: government
            }
        );

        let fed = colony.create(
            ColonyRow {
                name: "Fed".to_string(),
                population,
                food: five_days_worth,
            },
            ColonyLinks {
                body: get_body(),
                nation: government
            }
        );

        if let Some(fed) = colony.alloc.validate(fed) {
            colony.food_production.insert(fed, population.get_food_requirement());
        }

        (govt, colony)
    }

    fn get_body() -> Id<Body> {
        Allocator::<Body>::default().create()
    }
}