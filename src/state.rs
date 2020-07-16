use crate::star::Stars;
use crate::body::Bodies;
use crate::colony::Colonies;
use crate::nation::Nations;
use crate::time::{TimeState, DateTime};
use crate::systems::Systems;
use arena_ecs::Validates;

#[derive(Debug, Default)]
pub struct State {
    pub time: TimeState,
    pub systems: Systems,
    pub star: Stars,
    pub body: Bodies,
    pub nation: Nations,
    pub colony: Colonies,
}

impl State {
    pub fn new(start_date: DateTime) -> Self {
        Self {
            time: TimeState::new(start_date),
            .. Default::default()
        }
    }

    pub fn print(&self) {
        self.time.print();
        self.nation.print();
        self.colony.print(&self.nation);
    }
}

impl TimeState {
    fn print(&self) {
        println!("{}\n", self.get_time().format("%Y-%m-%d"));
    }
}

impl Nations {
    fn print(&self) {
        println!("\t- NATIONS -\n");

        let iter = self.name.iter()
            .zip(self.population.iter())
            .zip(self.agriculture.iter());

        self.alloc
            .zip_id_and_filter(iter)
            .for_each(|(((name, pop), agri), _)| {
                println!("{}\n  Pop: {}\n  Production: {:?}", name, pop.millions(), agri);
                println!();
            })
    }
}

impl Colonies {
    fn print(&self, nations: &Nations) {
        println!("\t- COLONIES -\n");

        let iter = self.name.iter()
            .zip(self.nation.iter())
            .zip(self.population.iter())
            .zip(self.food.iter())
            .zip(self.food_production.iter());

        self.alloc
            .zip_id_and_filter(iter)
            .for_each(|(((((name, nation), pop), food), food_prod), _)| {
                let nation = nations.alloc.validate(nation)
                    .map(|nation| nations.name.get(nation))
                    .expect("invalid nation");

                let food_cons = pop.get_food_requirement();

                println!("{} ({})", name, nation);
                println!("  Pop:       {}", pop.millions());
                println!("  Food:      {}", food.tons());
                println!("  Food Prod: {}", food_prod.tons_per_day());
                println!("  Food Cons: {}", food_cons.tons_per_day());
                println!("  Food Rsrv: {}", (food / food_cons).days());
                println!();
            });
    }
}