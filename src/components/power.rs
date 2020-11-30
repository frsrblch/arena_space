use super::{Duration, Energy, Force, Speed};

scalar! {
    struct Power(f64) {
        fn in_watts(watts) -> Self;
    }
}

scalar_div!(Energy | Duration = Power);
scalar_div!(Power | Speed = Force);

#[test]
fn conversion_test() {
    use crate::components::{KG, M, S};
    assert_eq!(Power::in_watts(1.0), 1.0 * KG * (M / S / S) * M / S);
}
