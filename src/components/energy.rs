use super::{Force, Length};

scalar! {
    struct Energy(f64) {
        fn in_joules(joules) -> Self;
    }
}

scalar_div!(Energy | Length = Force);

#[test]
fn conversion_test() {
    use crate::components::{KG, M, S};
    assert_eq!(Energy::in_joules(1.0), 1.0 * M / S / S * KG * M)
}
