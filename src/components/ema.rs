use std::ops::Mul;
use num_traits::MulAddAssign;

#[derive(Debug, Copy, Clone)]
pub struct ExpMovingAvg<T, const PERIOD: f64> {
    value: T,
}

impl<T: Default, const PERIOD: f64> Default for ExpMovingAvg<T, PERIOD> {
    fn default() -> Self {
        assert!(PERIOD >= 2.0, "ExpMovingAvg::PERIOD must be 2.0 or larger");
        Self {
            value: Default::default(),
        }
    }
}

impl<T, const PERIOD: f64> ExpMovingAvg<T, PERIOD>
    where
        T: Mul<f64,Output=T> + Copy + MulAddAssign<f64, T>,
{
    pub fn value(&self) -> T {
        self.value
    }

    pub fn add_next(&mut self, value: T) {
        self.value.mul_add_assign(Self::one_sub_multiplier(), value * Self::multiplier());
    }

    const fn multiplier() -> f64 {
        assert!(PERIOD >= 2.0);
        2.0 / (1.0 + PERIOD)
    }

    const fn one_sub_multiplier() -> f64 {
        1.0 - Self::multiplier()
    }
}

#[test]
fn test() {
    const PERIOD: f64 = 2.0;
    let mut ema = ExpMovingAvg::<f64, PERIOD>::default();

    ema.add_next(1.0);
    let expected_first = 2.0/(PERIOD + 1.0);

    assert_eq!(expected_first, ema.value());

    ema.add_next(3.0);
    let expected_second = expected_first * 1.0/(PERIOD + 1.0) + 2.0;

    assert_eq!(expected_second, ema.value());
}

#[test]
#[should_panic]
fn a() {
    let _ema = ExpMovingAvg::<f64, 1.0>::default();
}