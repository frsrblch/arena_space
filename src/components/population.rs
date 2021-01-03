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
    // MassRatePerPerson::in_kg_per_s_person(2.0 / Duration::SECONDS_PER_DAY);

    pub fn millions(self) -> Millions {
        Millions(self)
    }
}

#[test]
fn get_food_requirement() {
    let p = Population::in_millions(1.0);

    assert_eq!(p.get_food_requirement(), p * Population::FOOD_PER_PERSON);
}

pub struct Millions(Population);

impl Display for Millions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let millions = (self.0.value / 1e6) as i64;
        write!(f, "{} M", millions.to_formatted_string(&Locale::en))
    }
}

scalar!(struct PopulationDensity(f64));

impl PopulationDensity {
    pub const fn in_people_per_square_km(value: f64) -> Self {
        Self::new(value / 1e6)
    }
}

scalar_div! { Population | Area = PopulationDensity }
