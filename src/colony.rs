use crate::*;
use crate::body::Body;
use crate::government::Government;

#[derive(Debug, Default)]
pub struct Colony {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub population: Component<Self, Population>,
    pub food: Component<Self, Mass>,

    pub food_production: Component<Self, MassRate>,
    pub hunger: Component<Self, Fraction>,

    pub body: Component<Self, Id<Body>>,
    pub government: Component<Self, Id<Government>>,

    last_food_update: Time,
}

dynamic_arena!(Colony, u16);

impl Colony {
    pub fn create(&mut self, row: ColonyRow, links: ColonyLinks) -> Id<Self> {
        let id = self.alloc.create();

        self.name.insert(id, row.name);
        self.population.insert(id, row.population);
        self.food.insert(id, row.food);

        self.food_production.insert(id, MassRate::zero());
        self.hunger.insert(id, Fraction::default());

        self.body.insert(id, links.body);
        self.government.insert(id, links.government);

        id.id
    }

    pub fn delete(&mut self, id: Id<Self>) {
        if let Some(id) = self.alloc.validate(id) {
            self.population.insert(id, Population::zero());
            self.food_production.insert(id, MassRate::zero());
        }
    }

    pub fn get_food_demand(&self, id: Id<Self>) -> Option<MassRate> {
        self.get_population(id)
            .map(|pop| pop * Self::FOOD_PER_PERSON)
    }

    pub fn get_population(&self, id: Id<Self>) -> Option<&Population> {
        self.alloc
            .validate(id)
            .and_then(|id| self.population.get(id))
    }

    pub fn get_food(&self, id: Id<Self>) -> Option<&Mass> {
        self.alloc
            .validate(id)
            .and_then(|id| self.food.get(id))
    }

    pub fn sum_population(&self, id: impl Indexes<Government>) -> Population {
        self.alloc.living()
            .zip(self.government.iter())
            .zip(self.population.iter())
            .filter_map(|((living, govt), pop)| {
                if living && *govt == id.id() {
                    Some(*pop)
                } else {
                    None
                }
            })
            .sum()
    }

    /// 2 kg per person per day
    const FOOD_PER_PERSON: MassRatePerPerson = MassRatePerPerson::in_kg_per_s_person(
        2.0 / Duration::SECONDS_PER_DAY
    );

    pub fn update_food(&mut self, time: Time) {
        if time > self.next_food_update() {
            self.update_food_production_consumption();
        }
    }

    fn next_food_update(&self) -> Time {
        self.last_food_update + Self::FOOD_UPDATE_INTERVAL
    }

    fn update_food_production_consumption(&mut self) {
        self.food.iter_mut()
            .zip(self.hunger.iter_mut())
            .zip(self.food_production.iter())
            .zip(self.population.iter())
            .for_each(|(((food, hunger), production), pop)| {
                let consumption = pop * Self::FOOD_PER_PERSON;
                *food += (production - consumption) * Self::FOOD_UPDATE_INTERVAL;

                if *food < Mass::zero() {
                    *hunger = Fraction::new(-*food / (consumption * Self::FOOD_UPDATE_INTERVAL));
                    *food = Mass::zero();
                } else {
                    *hunger = Fraction::default();
                }
            });

        self.last_food_update += Self::FOOD_UPDATE_INTERVAL;
    }

    const FOOD_UPDATE_INTERVAL: Duration = Duration::in_s(1.0 * 3600.0 * 24.0);
}

#[derive(Debug, Clone)]
pub struct ColonyRow {
    pub name: String,
    pub population: Population,
    pub food: Mass,
}

#[derive(Debug, Copy, Clone)]
pub struct ColonyLinks {
    pub body: Id<Body>,
    pub government: Id<Government>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn food_consumption_hungry() {
        let mut time = Time::default();
        let (mut colony, id) = get_hungry_colony();

        while time < Time::in_days(10.0) {
            time += Duration::in_s(1.0 * 3600.0);

            colony.update_food(time);
        }

        assert_eq!(Mass::zero(), *colony.get_food(id).unwrap());
    }

    #[test]
    fn food_consumption_fed() {
        let mut time = Time::default();
        let (mut colony, id) = get_fed_colony();

        let starting_food = *colony.get_food(id).unwrap();

        while time < Time::in_days(10.0) {
            time += Duration::in_s(1.0 * 3600.0);

            colony.update_food(time);
        }

        let ending_food = *colony.get_food(id).unwrap();

        assert!(ending_food > starting_food);
    }

    fn get_hungry_colony() -> (Colony, Id<Colony>) {
        let mut colony = Colony::default();

        let id = colony.create(
            ColonyRow {
                name: "New Spaceville".to_string(),
                population: Population::in_millions(1.0),
                food: Mass::in_kg(10e6),
            },
            ColonyLinks {
                body: body(),
                government: govt()
            }
        );

        (colony, id)
    }

    fn get_fed_colony() -> (Colony, Id<Colony>) {
        let (mut colony, id) = get_hungry_colony();

        if let Some(id) = colony.alloc.validate(id) {
            if let Some(pop) = colony.population.get(id) {
                if let Some(production) = colony.food_production.get_mut(id) {
                    *production = pop * Colony::FOOD_PER_PERSON * 1.2;
                }
            }
        }

        (colony, id)
    }

    fn body() -> Id<Body> {
        Allocator::<Body>::default().create()
    }

    fn govt() -> Id<Government> {
        Allocator::<Government>::default().create().id()
    }
}