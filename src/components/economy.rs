use Resource::*;
use Facility::*;
use arena_ecs::{Component, ValidId, IdMap};
use std::slice::{Iter, IterMut};
use std::iter::Zip;
use std::fmt::{Display, Formatter};

array_enum!(
    Resource {
        Food,
        Ore,
        Metal,
    }
);

component_array!(ResourceComponent, Resource);

array_enum!(
    Facility {
        Farmland,
        Hydroponics,
        Mine,
        Foundry,
    }
);

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
            Foundry => &[Input { resource: Ore, multiplier: 4.0 }]
        }
    }

    pub const fn get_output(&self) -> Resource {
        match self {
            Farmland | Hydroponics => Food,
            Mine => Ore,
            Foundry => Metal,
        }
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

#[cfg(test)]
mod tests {
    array_enum!(Test { A, B, C });

    #[test]
    fn array_enum() {
        assert_eq!(0, Test::A.index());
        assert_eq!(1, Test::B.index());
        assert_eq!(2, Test::C.index());

        assert_eq!(3, Test::len());
    }
}