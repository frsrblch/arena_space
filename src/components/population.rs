use super::*;

pub const PERSON: Population = Population::new(1.0);

scalar!(struct Population(f64));

impl Population {
    pub const fn in_people(people: f64) -> Self {
        Self::new(people)
    }

    pub const fn in_millions(mm_people: f64) -> Self {
        Self::new(mm_people * 1e6)
    }

    pub const fn get_food_requirement(&self) -> MassRate {
        self * Self::FOOD_PER_PERSON
    }

    /// 2 kg per person per day
    const FOOD_PER_PERSON: MassRatePerPerson = 2.0 * KG / DAY / PERSON;
}

impl Display for Population {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.value.abs() {
            v if v < 1e4 => {
                write!(f, "{:.0}", self.value)
            }
            v if v < 1e5 => {
                write!(f, "{:.1} k", self.value / 1e3)
            }
            v if v < 1e6 => {
                write!(f, "{:.0} k", self.value / 1e3)
            }
            v if v < 1e7 => {
                write!(f, "{:.2} M", self.value / 1e6)
            }
            v if v < 1e8 => {
                write!(f, "{:.1} M", self.value / 1e6)
            }
            v if v < 1e9 => {
                write!(f, "{:.0} M", self.value / 1e6)
            }
            v if v < 1e10 => {
                write!(f, "{:.2} B", self.value / 1e9)
            }
            v if v < 1e11 => {
                write!(f, "{:.1} B", self.value / 1e9)
            }
            _ => write!(f, "{:.0} B", self.value / 1e9),
        }
    }
}

#[test]
fn population_to_string() {
    let assert_pop = |exp: &str, pop: f64| assert_eq!(exp, Population::new(pop).to_string());

    assert_pop("0", 0.0);
    assert_pop("1", 1e0);
    assert_pop("10", 1e1);
    assert_pop("100", 1e2);
    assert_pop("1000", 1e3);
    assert_pop("10.0 k", 1e4);
    assert_pop("100 k", 1e5);
    assert_pop("1.00 M", 1e6);
    assert_pop("10.0 M", 1e7);
    assert_pop("100 M", 1e8);
    assert_pop("1.00 B", 1e9);
    assert_pop("10.0 B", 1e10);
    assert_pop("100 B", 1e11);
    assert_pop("1000 B", 1e12);
    assert_pop("-10.0 k", -1e4);
}

scalar!(struct PopulationDensity(f64));

impl PopulationDensity {
    pub const fn in_people_per_square_km(value: f64) -> Self {
        Self::new(value / 1e6)
    }
}

scalar_div! { Population | Area = PopulationDensity }
