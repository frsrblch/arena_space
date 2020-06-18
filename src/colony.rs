use crate::*;
use crate::body::Body;
use crate::nation::Nation;

#[derive(Debug, Default)]
pub struct Colony {
    pub alloc: Allocator<Self>,

    pub name: Component<Self, String>,
    pub population: Component<Self, Population>,
    pub food: Component<Self, Mass>,

    pub food_production: Component<Self, MassRate>,
    pub hunger: Component<Self, Fraction>,

    pub body: Component<Self, Id<Body>>,
    pub nation: Component<Self, Option<Id<Nation>>>,

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
        self.nation.insert(id, Some(links.nation));

        id.id
    }

    pub fn delete(&mut self, id: Id<Self>) {
        if let Some(id) = self.alloc.validate(id) {
            self.population.insert(id, Population::zero());
            self.food.insert(id, Mass::zero());
            self.food_production.insert(id, MassRate::zero());
            self.hunger.insert(id, Fraction::default());
            self.nation.insert(id, None);

            let id = id.id;
            self.alloc.kill(id);
        }
    }
}

mod population {
    use super::*;

    impl Colony {
        pub fn get_population(&self, id: Id<Self>) -> Option<&Population> {
            self.alloc.validate(id)
                .and_then(|id| self.population.get(id))
        }
    }
}

mod food {
    use super::*;

    impl Colony {
        pub fn get_food(&self, id: Id<Self>) -> Option<&Mass> {
            self.alloc.validate(id)
                .and_then(|id| self.food.get(id))
        }

        pub fn update_food(&mut self, time: Time) {
            while time > self.next_food_update() {
                self.update_food_and_hunger();

                self.last_food_update += Self::FOOD_UPDATE_INTERVAL;
            }
        }

        fn next_food_update(&self) -> Time {
            self.last_food_update + Self::FOOD_UPDATE_INTERVAL
        }

        fn update_food_and_hunger(&mut self) {
            self.food.iter_mut()
                .zip(self.hunger.iter_mut())
                .zip(self.food_production.iter())
                .zip(self.population.iter())
                .for_each(|(((food, hunger), production), pop)| {
                    let consumption = pop.get_food_requirement();
                    *food += (production - consumption) * Self::FOOD_UPDATE_INTERVAL;

                    if *food < Mass::zero() {
                        *hunger = Fraction::new(-*food / (consumption * Self::FOOD_UPDATE_INTERVAL));
                        *food = Mass::zero();
                    } else {
                        *hunger = Fraction::default();
                    }
                });
        }

        const FOOD_UPDATE_INTERVAL: Duration = Duration::in_s(1.0 * 3600.0 * 24.0);
    }
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
    pub nation: Id<Nation>,
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
                nation: govt()
            }
        );

        (colony, id)
    }

    fn get_fed_colony() -> (Colony, Id<Colony>) {
        let (mut colony, id) = get_hungry_colony();

        if let Some(id) = colony.alloc.validate(id) {
            if let Some(pop) = colony.population.get(id) {
                if let Some(production) = colony.food_production.get_mut(id) {
                    *production = pop.get_food_requirement() * 1.2;
                }
            }
        }

        (colony, id)
    }

    fn body() -> Id<Body> {
        Allocator::<Body>::default().create()
    }

    fn govt() -> Id<Nation> {
        Allocator::<Nation>::default().create().id()
    }
}