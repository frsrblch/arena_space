use super::*;

array_enum! {
    enum Facility {
        type Array = struct FacilityArray;
        Farmland,
        Hydroponics,
        Mine,
        Foundry,
    }
}

component_map!(FacilityMap, Facility);

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
            .reduce(|a, b| a + b)
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
