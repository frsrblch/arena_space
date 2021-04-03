use crate::body::{Bodies, Body};
use crate::colony::economy::{Production, Resources};
use crate::colony::population::People;
use crate::systems::System;
use crate::*;

pub mod economy;
mod population;

#[derive(Debug, Clone)]
pub struct Colony {
    pub name: String,
    pub population: Population,
}

fixed_arena!(Colony);

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
}

#[derive(Debug, Default)]
pub struct Colonies {
    pub alloc: Allocator<Colony>,

    pub name: Component<Colony, String>,

    pub people: People,
    pub resources: Resources,
    pub production: Production,

    pub body: Component<Colony, Id<Body>>,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.people.insert(id, row.population);
        self.resources.insert(id);

        self.body.insert(id, links.body);

        id
    }

    pub fn print(&self) {
        println!(" == COLONIES ==");
        for id in self.alloc.ids() {
            self.print_colony(id);
        }
    }

    fn print_colony<I: ValidId<Colony>>(&self, id: I) {
        println!("{}: {}", self.name.get(id), self.people.population.get(id));
        self.resources.print_colony(id);
        self.production.print_colony(id);
    }

    pub fn get_body<I: ValidId<Colony>>(&self, id: I) -> Id<Body> {
        *self.body.get(id)
    }
}

pub mod examples {
    use super::*;

    pub fn european_union() -> Colony {
        Colony {
            name: "European Union".to_string(),
            population: Population::in_millions(623.3),
        }
    }

    pub fn china() -> Colony {
        Colony {
            name: "People's Republic of China".to_string(),
            population: Population::in_millions(1.531e3),
        }
    }
}
