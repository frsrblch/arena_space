use crate::body::{Bodies, Body};
use crate::nation::Nation;
use crate::systems::System;
use crate::*;
use crate::colony::economy::{Resources, Production};
use crate::colony::population::People;

mod population;
pub mod economy;

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

type Satiation = ExpMovingAvg<f64, 15.0>;

#[derive(Debug, Default)]
pub struct Colonies {
    pub alloc: Allocator<Colony>,

    pub name: Component<Colony, String>,

    pub people: People,
    pub resources: Resources,
    pub production: Production,

    pub body: Component<Colony, Id<Body>>,
    pub nation: Component<Colony, Option<Id<Nation>>>,
}

impl Colonies {
    pub fn create(&mut self, row: Colony, links: ColonyLinks) -> Id<Colony> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);

        self.people.insert(id, row.population);
        self.resources.insert(id);

        self.body.insert(id, links.body);
        self.nation.insert(id, Some(links.nation));

        id.id
    }

    pub fn kill(&mut self, id: Id<Colony>) {
        if let Some(id) = self.alloc.validate(id) {
            self.name.get_mut(id).clear();


            self.people.insert(id, Population::zero());
            self.resources.insert(id);
            self.production.kill(id.id);

            self.nation.insert(id, None);

            let id = id.id;
            self.alloc.kill(id);
        }
    }
}