use super::*;
use crate::body::{BodyProperties, Habitability};

array_enum! {
    enum Resource {
        type Array = struct ResourceArray;
        Food,
        Ore,
        Metal,
    }
}

pub const PRICE_DEFAULT: ResourceArray<Price> = ResourceArray::new([
    Price::in_credits_per_kg(1.0), // Food
    Price::in_credits_per_kg(1.0), // Ore
    Price::in_credits_per_kg(4.0), // Metal
]);

component_array!(ResourceComponent, Resource, ResourceArray);

impl<ID> ResourceComponent<ID, Price> {
    pub fn insert_default_prices<I: ValidId<ID>>(&mut self, id: I) {
        self.iter_mut()
            .zip(PRICE_DEFAULT.iter())
            .for_each(|(prices, default)| prices.insert(id, *default));
    }
}

impl<ID, T: Default> ResourceComponent<ID, T> {
    pub fn insert_default<I: ValidId<ID>>(&mut self, id: I) {
        self.iter_mut()
            .for_each(|values| values.insert(id, T::default()));
    }
}

impl Resource {
    pub fn get_production_cost(&self, location: &BodyProperties) -> Price {
        match self {
            Food => match location.get_habitability() {
                Habitability::Uninhabitable => Price::in_credits_per_kg(8.0),
                Habitability::Hostile => Price::in_credits_per_kg(4.0),
                Habitability::Marginal => Price::in_credits_per_kg(1.0),
                Habitability::Optimal => Price::in_credits_per_kg(0.5),
            },
            Ore => Price::in_credits_per_kg(0.1),
            Metal => Price::in_credits_per_kg(5.0),
        }
    }

    pub const fn get_facility(&self) -> &'static [Facility] {
        match self {
            Food => &[Farmland, Hydroponics],
            Ore => &[Mine],
            Metal => &[Foundry],
        }
    }

    pub const fn get_annual_decay(&self) -> Option<f64> {
        match self {
            Food => Some(0.925),
            Ore => None,
            Metal => None,
        }
    }

    pub fn get_default_price(&self) -> Price {
        self.get_facility()
            .iter()
            .map(|f| f.get_default_price())
            .reduce(|a, b| if a < b { a } else { b })
            .unwrap_or_else(|| Price::in_credits_per_kg(1.0))
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Food => "Food",
            Ore => "Ore",
            Metal => "Metal",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Input {
    pub resource: Resource,
    pub multiplier: f64,
}

impl Input {
    pub fn get_default_price(&self) -> Price {
        self.resource.get_default_price() * self.multiplier
    }
}
