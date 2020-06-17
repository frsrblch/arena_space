use crate::*;
use crate::body::Body;
use crate::government::Government;

#[derive(Debug, Default)]
pub struct Colony {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub population: Component<Self, Population>,

    pub body: Component<Self, Id<Body>>,
    pub government: Component<Self, Id<Government>>,
}

dynamic_arena!(Colony, u16);

impl Colony {
    pub fn create(&mut self, row: ColonyRow, links: ColonyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);

        self.body.insert(id, links.body);
        self.government.insert(id, links.government);

        id.id
    }

    pub fn get_food_demand(&self, id: Id<Self>) -> Option<MassRate> {
        self.get_population(id)
            .map(|pop| pop * Self::FOOD_RATE_PER_PERSON)
    }

    pub fn get_population(&self, id: Id<Self>) -> Option<&Population> {
        self.alloc
            .validate(id)
            .and_then(|id| self.population.get(id))
    }

    const FOOD_RATE_PER_PERSON: MassRatePerPerson = MassRatePerPerson::in_kg_per_s_person(
        2.0 / Duration::SECONDS_PER_DAY
    ); // 2 kg per person per day
}

#[derive(Debug, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: Population,
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub government: Id<Government>,
}