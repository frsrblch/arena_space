use crate::components::Price;
use gen_id::{Component, IdMap, ValidId};
use std::fmt::{Display, Formatter};
use typed_iter::{IterOver, Zip};
use Facility::*;
use Resource::*;

array_enum!(ResourceArray Resource { Food, Ore, Metal });

pub const PRICE_DEFAULT: ResourceArray<Price> = ResourceArray::new([
    Price::in_credits_per_kg(1.0), // Food
    Price::in_credits_per_kg(1.0), // Ore
    Price::in_credits_per_kg(4.0), // Metal
]);

component_array!(ResourceComponent, Resource, ResourceArray);

impl<ID, T> ResourceComponent<ID, T> {
    pub fn zip<U: IterOver<Type = Resource>>(&self, rhs: U) -> Zip<Resource, &Self, U> {
        <&Self as IterOver>::zip(self, rhs)
    }

    pub fn zip_mut<U: IterOver<Type = Resource>>(&mut self, rhs: U) -> Zip<Resource, &mut Self, U> {
        <&mut Self as IterOver>::zip(self, rhs)
    }
}

// TODO use typed_iterator Iter and IterMut types for enum array types

impl<'a, ID, T> IntoIterator for &'a ResourceComponent<ID, T> {
    type Item = &'a Component<ID, T>;
    type IntoIter = std::slice::Iter<'a, Component<ID, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
    }
}

impl<'a, ID, T> IterOver for &'a ResourceComponent<ID, T> {
    type Type = Resource;
}

impl<'a, ID, T> IntoIterator for &'a mut ResourceComponent<ID, T> {
    type Item = &'a mut Component<ID, T>;
    type IntoIter = std::slice::IterMut<'a, Component<ID, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter_mut()
    }
}

impl<'a, ID, T> IterOver for &'a mut ResourceComponent<ID, T> {
    type Type = Resource;
}

array_enum!(FacilityArray Facility {
    Farmland,
    Hydroponics,
    Mine,
    Foundry,
});

impl<ID> ResourceComponent<ID, Price> {
    pub fn insert_default_prices<I: ValidId<ID>>(&mut self, id: I) {
        for (prices, resource) in self.iter_enum_mut() {
            let price = PRICE_DEFAULT[*resource];
            prices.insert(id, price);
        }
    }
}

component_map!(FacilityMap, Facility);

impl Resource {
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
            .fold_first(|a, b| if a < b { a } else { b })
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

impl Facility {
    pub const fn get_inputs(&self) -> &'static [Input] {
        match self {
            Farmland | Hydroponics | Mine => &[],
            Foundry => &[Input {
                resource: Ore,
                multiplier: 4.0,
            }],
        }
    }

    pub const fn get_output(&self) -> Resource {
        match self {
            Farmland | Hydroponics => Food,
            Mine => Ore,
            Foundry => Metal,
        }
    }

    pub fn get_default_price(&self) -> Price {
        self.get_inputs()
            .iter()
            .map(|i| i.get_default_price())
            .fold_first(|a, b| a + b)
            .unwrap_or_else(|| Price::in_credits_per_kg(1.0))
    }
}

impl Display for Facility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Farmland => "Farmland",
            Hydroponics => "Hydroponics",
            Mine => "Mine",
            Foundry => "Foundry",
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

#[cfg(test)]
mod tests {
    use super::*;

    array_enum!(TestArray Test { A, B, C });

    #[test]
    fn array_enum() {
        assert_eq!(0, Test::A.index());
        assert_eq!(1, Test::B.index());
        assert_eq!(2, Test::C.index());

        assert_eq!(3, Test::len());
    }

    #[test]
    fn resource_get_default_price() {
        assert_eq!(
            Price::in_credits_per_kg(1.0),
            Resource::Food.get_default_price()
        );
        assert_eq!(
            Price::in_credits_per_kg(1.0),
            Resource::Ore.get_default_price()
        );
        assert_eq!(
            Price::in_credits_per_kg(4.0),
            Resource::Metal.get_default_price()
        );
    }

    #[test]
    fn facility_get_default_price() {
        assert_eq!(
            Price::in_credits_per_kg(1.0),
            Facility::Mine.get_default_price()
        );
        assert_eq!(
            Price::in_credits_per_kg(1.0),
            Facility::Hydroponics.get_default_price()
        );
        assert_eq!(
            Price::in_credits_per_kg(1.0),
            Facility::Farmland.get_default_price()
        );
        assert_eq!(
            Price::in_credits_per_kg(4.0),
            Facility::Foundry.get_default_price()
        );
    }

    #[test]
    fn price_default_array_values() {
        PRICE_DEFAULT
            .into_iter()
            .zip(Resource::array().iter())
            .for_each(|(price, resource)| {
                assert_eq!(*price, resource.get_default_price());
            });
    }
}
