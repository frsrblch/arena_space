use super::{Acceleration, Mass};

scalar! {
    struct Force(f64) {
        fn in_newtons(newtons) -> Self;
    }
}

scalar_div!(Force | Acceleration = Mass);

#[test]
fn conversion() {
    use crate::components::{KG, M, S};
    assert_eq!(Force::in_newtons(1.0), 1.0 * M / S / S * KG)
}
