use crate::components::{DurationFloat, MassRatePerPerson, Population};
use std::fmt::{Display, Formatter, Result};

scalar! {
    struct Credits(f64) {
        fn in_credits(credits) -> Self;
    }
}

scalar! {
    struct CreditRate(f64) {
        fn in_credits_per_s(credits_per_second) -> Self;
    }
}

scalar_div! { Credits | DurationFloat = CreditRate }

scalar! {
    struct CreditsPerPerson(f64) {
        fn in_credits_per_person(credits_per_person) -> Self;
    }
}

scalar_div! { Credits | Population = CreditsPerPerson }

scalar! {
    struct CreditsPerSecondPerPerson(f64) {
        fn in_credits_per_s_person(credits_per_second_person) -> Self;
    }
}

pub type Wage = CreditsPerSecondPerPerson;

scalar_div! { CreditRate | Population = CreditsPerSecondPerPerson }
scalar_div! { CreditsPerPerson | DurationFloat = CreditsPerSecondPerPerson }

scalar! {
    struct CreditsPerKilogram(f64) {
        fn in_credits_per_kg(credits_per_kilogram) -> Self;
    }
}

pub type Price = CreditsPerKilogram;

impl Display for Price {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "${:.2}/kg", self.value)
    }
}

#[test]
fn price_display() {
    let p = Price::in_credits_per_kg(3.333333);
    assert_eq!(&format!("{}", p), "$3.33/kg");
}

scalar_div! { CreditsPerSecondPerPerson | MassRatePerPerson = CreditsPerKilogram }