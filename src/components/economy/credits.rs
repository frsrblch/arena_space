use crate::components::{Duration, Length, Mass, MassRatePerPerson, Population};
use std::fmt::{Display, Formatter, Result};

pub const CR: Credits = Credits::in_credits(1.0);

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

scalar_div! { Credits | Duration = CreditRate }

scalar! {
    struct PricePerMeter(f64) {
        fn in_credits_per_kg_m(credits_per_kilogram_meter) -> Self;
    }
}

scalar_div! { Price | Length = PricePerMeter }

scalar! {
    struct CreditsPerMeter(f64) {
        fn in_credits_per_m(credits_per_meter) -> Self;
    }
}

scalar_div! { Credits | Length = CreditsPerMeter }
scalar_div! { CreditsPerMeter | Mass = PricePerMeter }

scalar! {
    struct CreditsPerPerson(f64) {
        fn in_credits_per_person(credits_per_person) -> Self;
    }
}

scalar_div! { Credits | Population = CreditsPerPerson }

scalar! {
    struct CreditRatePerPerson(f64) {
        fn in_credits_per_s_person(credits_per_second_person) -> Self;
    }
}

pub type Wage = CreditRatePerPerson;

scalar_div! { CreditRate | Population = CreditRatePerPerson }
scalar_div! { CreditsPerPerson | Duration = CreditRatePerPerson }

scalar! {
    struct Price(f64) {
        fn in_credits_per_kg(credits_per_kilogram) -> Self;
    }
}

scalar_div! { Credits | Mass = Price }

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

scalar_div! { CreditRatePerPerson | MassRatePerPerson = Price }
