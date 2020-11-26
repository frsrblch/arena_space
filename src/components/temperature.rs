pub const K: Temperature = Temperature::in_k(1.0);

scalar! {
    struct Temperature(f64) {
        fn in_k(kelvin) -> Self;
    }
}
