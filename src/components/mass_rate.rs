use super::*;

scalar!(MassRate, kg_per_second, in_kg_per_s);

impl MassRate {
    pub fn in_tons_per_day(tons: f64) -> Self {
        Self::in_kg_per_s(tons * 1000.0 / (24.0 * 3600.0))
    }

    pub fn tons_per_day(self) -> TonsPerDay {
        TonsPerDay(self)
    }
}

pub struct TonsPerDay(MassRate);

impl Display for TonsPerDay {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tons_per_day = (self.0.value / 1e3 * DurationFloat::SECONDS_PER_DAY) as i64;
        write!(f, "{} t/day", tons_per_day.to_formatted_string(&Locale::en))
    }
}

scalar_div!(Mass, DurationFloat, MassRate);

scalar!(MassRatePerPerson, kg_per_person_second, in_kg_per_s_person);
pub type Productivity = MassRatePerPerson;
scalar_div!(MassRate, Population, MassRatePerPerson);
