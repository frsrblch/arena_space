macro_rules! vector_and_scalar {
    {
        struct $vector:ident([struct $scalar:ident($base:ty);2]) {
            fn $abrev:ident($unit:ident) -> Self;
        }
    } => {
        scalar! {
            struct $scalar($base) {
                fn $abrev($unit) -> Self;
            }
        }
        vector! {
            struct $vector([$scalar; 2]) {
                fn $abrev($unit: $base) -> Self;
            }
        }

        impl std::ops::Mul<$scalar> for ($base, $base) {
            type Output = $vector;

            fn mul(self, rhs: $scalar) -> Self::Output {
                $vector {
                    x: self.0 * rhs,
                    y: self.1 * rhs,
                }
            }
        }

        impl std::ops::Mul<&$scalar> for ($base, $base) {
            type Output = $vector;

            fn mul(self, rhs: &$scalar) -> Self::Output {
                self * *rhs
            }
        }
    };
}
