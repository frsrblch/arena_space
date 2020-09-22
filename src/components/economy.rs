use Resource::*;
use Facility::*;
use arena_ecs::{Component, Indexes, IdMap};
use std::iter::Zip;
use std::slice::{Iter, IterMut};

array_enum!(
    Resource {
        Food,
        Ore,
        Metal,
    }
);

array_enum!(
    Facility {
        Farmland,
        Hydroponics,
        Mine,
        Foundry,
    }
);

impl Resource {
    pub const fn get_facility(&self) -> &'static [Facility] {
        match self {
            Food => &[Farmland, Hydroponics],
            Ore => &[Mine],
            Metal => &[Foundry],
        }
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Input {
    pub resource: Resource,
    pub multiplier: f64,
}

#[derive(Debug)]
pub struct ResourceComponent<ID, T> {
    components: [Component<ID, T>; Resource::len()],
}

impl<ID, T> Default for ResourceComponent<ID, T> {
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}

impl<ID, T: Default + Clone> ResourceComponent<ID, T> {
    pub fn get(&self, resource: Resource) -> &Component<ID, T> {
        &self.components[usize::from(resource)]
    }

    pub fn get_mut(&mut self, resource: Resource) -> &mut Component<ID, T> {
        &mut self.components[usize::from(resource)]
    }

    pub fn insert<I: Indexes<ID>>(&mut self, id: I, value: T) {
        self.components
            .iter_mut()
            .for_each(|comp| comp.insert(id, value.clone()));
    }

    pub fn iter(&self) -> Iter<Component<ID, T>> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Component<ID, T>> {
        self.components.iter_mut()
    }

    pub fn fill_with<F: Fn() -> T + Copy>(&mut self, f: F) {
        self.components
            .iter_mut()
            .for_each(|comp| comp.fill_with(f));
    }
}

#[derive(Debug)]
pub struct FacilityMap<ID, T> {
    map: [IdMap<ID, T>; Facility::len()],
}

impl<ID, T> Default for FacilityMap<ID, T> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<ID, T> FacilityMap<ID, T> {
    pub fn get(&self, facility: Facility) -> &IdMap<ID, T> {
        &self.map[usize::from(facility)]
    }

    pub fn get_mut(&mut self, facility: Facility) -> &mut IdMap<ID, T> {
        &mut self.map[usize::from(facility)]
    }

    pub fn iter(&self) -> Zip<Iter<IdMap<ID, T>>, Iter<Facility>> {
        self.map.iter()
            .zip(Facility::array())
    }

    pub fn iter_mut(&mut self) -> Zip<IterMut<IdMap<ID, T>>, Iter<Facility>> {
        self.map.iter_mut()
            .zip(Facility::array())
    }
}

#[cfg(test)]
mod tests {
    array_enum!(Test { A, B, C });

    #[test]
    fn array_enum() {
        assert_eq!(0, usize::from(Test::A));
        assert_eq!(1, usize::from(Test::B));
        assert_eq!(2, usize::from(Test::C));

        assert_eq!(3, Test::len());
    }
}