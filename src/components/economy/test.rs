
use super::*;

array_enum! {
    enum Test {
        A,
        B,
        C,
    }
}

#[test]
fn array_enum() {
    assert_eq!(0, Test::A.index());
    assert_eq!(1, Test::B.index());
    assert_eq!(2, Test::C.index());

    assert_eq!(3, Test::LEN);
}

#[test]
fn resource_get_default_price() {
    assert_eq!(
        Price::in_credits_per_kg(1.0),
        Resource::Food.get_default_price()
    );
    assert_eq!(
        Price::in_credits_per_kg(1.0),
        Resource::Ore.get_default_price()
    );
    assert_eq!(
        Price::in_credits_per_kg(4.0),
        Resource::Metal.get_default_price()
    );
}

#[test]
fn facility_get_default_price() {
    assert_eq!(
        Price::in_credits_per_kg(1.0),
        Facility::Mine.get_default_price()
    );
    assert_eq!(
        Price::in_credits_per_kg(1.0),
        Facility::Hydroponics.get_default_price()
    );
    assert_eq!(
        Price::in_credits_per_kg(1.0),
        Facility::Farmland.get_default_price()
    );
    assert_eq!(
        Price::in_credits_per_kg(4.0),
        Facility::Foundry.get_default_price()
    );
}

#[test]
fn price_default_array_values() {
    PRICE_DEFAULT
        .iter()
        .zip(Resource::iter())
        .for_each(|(price, resource)| {
            assert_eq!(*price, resource.get_default_price());
        });
}
