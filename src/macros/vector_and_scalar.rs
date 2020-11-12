macro_rules! vector_and_scalar {
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident, $base:ty) => {
        scalar!($scalar, $unit, $abrev, $base);
        vector!($vector, $scalar, $unit, $abrev, $base);
    };
    ($vector:ident, $scalar:ident, $unit:ident, $abrev:ident) => {
        vector_and_scalar!($vector, $scalar, $unit, $abrev, f64);
    };
}
