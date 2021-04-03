#![allow(dead_code)]

mod trade;

macro_rules! combined_enum {
    {
        enum $name:ident {
            $(
                $variant:ident,
            )*
        }
    } => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum $name {
            $(
                $variant($variant),
            )*
        }

        $(
            impl From<$variant> for $name {
                fn from(value: $variant) -> $name {
                    $name::$variant(value)
                }
            }
        )*

        impl $name {
            pub const LEN: usize = 0 $( + $variant::LEN )* ;
        }

        #[test]
        fn index_overlap() {
            let mut indices = std::collections::HashSet::<usize>::default();

            $(
                for &value in $variant::iter() {
                    let index = Resource::$variant(value).index();

                    assert!(index < Resource::LEN);
                    assert!(!indices.contains(&index));

                    indices.insert(index);
                }
            )*

            assert_eq!(indices.len(), Resource::LEN);
        }
    }
}

combined_enum! {
    enum Resource {
        Mineral,
        Metal,
        Agricultural,
        Product,
        Fuel,
    }
}

impl Resource {
    pub const fn index(&self) -> usize {
        match self {
            Self::Mineral(mineral) => mineral.index(),
            Self::Metal(metal) => metal.index() + Mineral::LEN,
            Self::Agricultural(agricultural) => agricultural.index() + Mineral::LEN + Metal::LEN,
            Self::Product(product) => {
                product.index() + Mineral::LEN + Metal::LEN + Agricultural::LEN
            }
            Self::Fuel(fuel) => {
                fuel.index() + Mineral::LEN + Metal::LEN + Agricultural::LEN + Product::LEN
            }
        }
    }
}

array_enum! {
    enum Mineral {
        Hematite,
        Bauxite,
        Rutile,
        Chalcopyrite,
        Uraninite,
        Pentlandite,
        Chromite,
    }
}

impl Mineral {
    pub fn refine(&self) -> Resource {
        match self {
            Mineral::Hematite => Metal::Steel.into(),
            Mineral::Bauxite => Metal::Aluminum.into(),
            Mineral::Rutile => Metal::Titanium.into(),
            Mineral::Chalcopyrite => Metal::Copper.into(),
            Mineral::Uraninite => Fuel::Uranium.into(),
            Mineral::Pentlandite => Metal::Nickel.into(),
            Mineral::Chromite => Metal::Chromium.into(),
        }
    }
}

array_enum! {
    enum Metal {
        Steel,
        Aluminum,
        Titanium,
        Copper,
        Nickel,
        Chromium,
    }
}

impl Metal {
    fn ore(&self) -> &'static [Mineral] {
        match self {
            Metal::Steel => &[Mineral::Hematite],
            Metal::Aluminum => &[Mineral::Bauxite],
            Metal::Titanium => &[Mineral::Rutile],
            Metal::Copper => &[Mineral::Chalcopyrite],
            Metal::Nickel => &[Mineral::Pentlandite],
            Metal::Chromium => &[Mineral::Chromite],
        }
    }
}

array_enum! {
    enum Agricultural {
        Grain,
        Produce,
        AnimalProducts,
        Textiles,
        Lumber,
    }
}

array_enum! {
    enum Product {
        ConsumerGoods,
        MiningEquipment,
        FarmingEquipment,
        IndustrialEquipment,
        MilitaryEquipment,
    }
}

array_enum! {
    enum Fuel {
        Chemical,
        Uranium,
        Deuterium,
        Antimatter,
    }
}
