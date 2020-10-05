use super::*;
use crate::colony::Colonies;
use crate::nation::economy::Economy;

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

    pub economy: Economy,
}

impl Nations {
    pub fn create(&mut self, row: Nation) -> Id<Nation> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, Population::default());
        self.economy.insert(id);

        id.value
    }
}

mod food_production_targets {
    use super::*;

    impl Nations {
        pub fn update_food_targets(&mut self, colonies: &Colonies) {
            self.sum_population_from(colonies);
            // unimplemented!();
        }

        fn sum_population_from(&mut self, colonies: &Colonies) {
            self.population.sum_from_opt(
                &colonies.people.population,
                &colonies.nation,
                &self.alloc,
            );
        }
    }
}

mod economy {
    use crate::colony::Colonies;
    use crate::components::{MassRate, ResourceComponent};
    use crate::nation::Nation;
    use arena_ecs::{Allocator, ValidId};

    #[derive(Debug, Default)]
    pub struct Economy {
        pub production: ResourceComponent<Nation, MassRate>,
        pub consumption: ResourceComponent<Nation, MassRate>,
    }

    impl Economy {
        pub fn insert<I: ValidId<Nation>>(&mut self, id: I) {
            self.production.insert(id, MassRate::zero());
            self.consumption.insert(id, MassRate::zero());
        }

        pub fn sum_from(&mut self, colonies: &mut Colonies, allocator: &Allocator<Nation>) {
            self.sum_production(colonies, allocator);
            self.sum_consumption(colonies, allocator);
        }

        fn sum_production(&mut self, colonies: &mut Colonies, allocator: &Allocator<Nation>) {
            self.production.fill_with(MassRate::zero);

            let iter = self
                .production
                .iter_mut()
                .zip(colonies.production.iter_mut());

            let colony_nation = colonies.nation.validate(allocator);

            for (national_production, colony_production) in iter {
                let colony_production = colony_production.validate(&colonies.alloc);

                for (colony, unit) in colony_production.iter() {
                    if let Some(nation) = colony_nation.get(colony) {
                        let production = national_production.get_mut(nation);
                        *production += unit.capacity;
                    }
                }
            }
        }

        fn sum_consumption(&mut self, colonies: &mut Colonies, allocator: &Allocator<Nation>) {
            self.consumption.fill_with(MassRate::zero);

            let colony_nation = colonies.nation.validate(allocator);

            let iter = self
                .consumption
                .iter_mut()
                .zip(colonies.resources.requested.iter());

            for (consumption, requested) in iter {
                let iter = requested.iter().zip(colony_nation.iter());

                for (requested, nation) in iter {
                    if let Some(nation) = nation {
                        let consumption = consumption.get_mut(nation);
                        *consumption += requested;
                    }
                }
            }
        }
    }
}
