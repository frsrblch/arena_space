use std::ops::Mul;
use num_traits::MulAddAssign;


#[derive(Debug, Default, Copy, Clone)]
pub struct Ema<T, const PERIOD: f64> {
    value: T,
}

impl<T, const PERIOD: f64> Ema<T, PERIOD>
    where T: Mul<f64,Output=T> + Copy + MulAddAssign<f64, T>,
{
    pub fn value(&self) -> T {
        self.value
    }

    pub fn add_next(&mut self, value: T) {
        self.value.mul_add_assign(1.0 - Self::multiplier(), value * Self::multiplier());
    }

    const fn multiplier() -> f64 {
        2.0 / (1.0 + PERIOD)
    }
}

#[test]
fn test() {
    const PERIOD: f64 = 2.0;
    let mut ema = Ema::<f64, PERIOD>::default();

    ema.add_next(1.0);
    let expected_first = 2.0/(PERIOD + 1.0);

    assert_eq!(expected_first, ema.value());

    ema.add_next(3.0);
    let expected_second = expected_first * 1.0/(PERIOD + 1.0) + 2.0;

    assert_eq!(expected_second, ema.value());
}
