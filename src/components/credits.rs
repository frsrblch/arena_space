use super::{DurationFloat, MassRatePerPerson, Population};

scalar!(Credits, credits, in_credits);

scalar!(CreditRate, credits_per_second, in_credits_per_s);
scalar_div!(Credits, DurationFloat, CreditRate);

scalar!(CreditsPerPerson, credits_per_person, in_credits_per_person);
scalar_div!(Credits, Population, CreditsPerPerson);

scalar!(
    CreditsPerSecondPerPerson,
    credits_per_second_person,
    in_credits_per_s_person
);
pub type Wage = CreditsPerSecondPerPerson;
scalar_div!(CreditRate, Population, CreditsPerSecondPerPerson);
scalar_div!(CreditsPerPerson, DurationFloat, CreditsPerSecondPerPerson);

scalar!(CreditsPerKilogram, credits_per_kilogram, in_credits_per_kg);
pub type Price = CreditsPerKilogram;
scalar_div!(
    CreditsPerSecondPerPerson,
    MassRatePerPerson,
    CreditsPerKilogram
);
