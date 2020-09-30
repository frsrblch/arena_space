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
}

impl Nations {
    pub fn create(&mut self, row: Nation) -> Id<Nation> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.population.insert(id, Population::default());

        id.value
    }
}

mod food_production_targets {
    use super::*;

    impl Nations {
        pub fn update_food_targets(&mut self, colonies: &Colonies) {
            self.sum_population_from(colonies);
            unimplemented!();
        }

        fn sum_population_from(&mut self, colonies: &Colonies) {
            self.population.sum_from_opt(&colonies.people.population, &colonies.nation, &self.alloc);
        }
    }
}