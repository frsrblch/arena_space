#![allow(dead_code)] // TODO remove when stabilized

use crate::components::*;

#[derive(Debug, Copy, Clone)]
pub enum Propulsion {
    Warp(WarpFactor),
}

impl Propulsion {
    pub const LEN: usize = 1;

    pub const DEFAULTS: [Self; Self::LEN] = [Self::Warp(Default::default())];

    pub fn get_duration(&self, distance: Length) -> Option<Duration> {
        match self {
            Propulsion::Warp(factor) => Some(distance / factor.get_max_speed()),
        }
    }

    pub fn iter_defaults() -> std::slice::Iter<'static, Propulsion> {
        Self::DEFAULTS.iter()
    }

    pub fn index(&self) -> usize {
        match self {
            Propulsion::Warp(_) => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct WarpFactor(u32);

#[rustfmt::skip]
impl const Default for WarpFactor {
    fn default() -> Self {
        WarpFactor(1000)
    }
}

impl WarpFactor {
    pub fn new(fraction_of_c: f64) -> Self {
        Self((fraction_of_c * 1000.0) as u32)
    }

    pub fn get_max_speed(&self) -> Speed {
        Speed::C * (self.0 as f64 / 1000.0)
    }
}

impl From<Speed> for WarpFactor {
    fn from(speed: Speed) -> Self {
        WarpFactor::new(speed / Speed::C)
    }
}

#[test]
fn warp_factor_new() {
    assert_eq!(WarpFactor(0), WarpFactor::from(Speed::zero()));
    assert_eq!(WarpFactor(1000), WarpFactor::from(Speed::C));
    assert_eq!(WarpFactor(2000), WarpFactor::from(2.0 * Speed::C));
}

#[test]
fn warp_factor_get_max_speed() {
    assert_eq!(WarpFactor(0).get_max_speed(), Speed::zero());
    assert_eq!(WarpFactor(1000).get_max_speed(), Speed::C);
    assert_eq!(WarpFactor(2000).get_max_speed(), 2.0 * Speed::C);
}

pub struct FtlProperties {
    pub min_drive_size: Mass,
    pub relative_size: f64,
    pub energy_requirement: (),
}

#[derive(Debug, Copy, Clone)]
pub enum FTLDrive {
    Warp(WarpDrive),
    Jump(JumpDrive),
}

/// Bends spacetime around the vessel. Requires constant power to operate.
/// P / P_max = (a / a_max) ^ (3/2)
#[derive(Debug, Copy, Clone)]
pub struct WarpDrive {
    pub max_acceleration: Acceleration,
    pub max_power_draw: Power,
}

impl WarpDrive {
    pub fn new(max_acceleration: Acceleration, max_power_draw: Power) -> Self {
        Self {
            max_acceleration,
            max_power_draw,
        }
    }
}

impl WarpDrive {
    pub fn get_trip_duration(&self, distance: Length, power_fraction: Fraction) -> Duration {
        let accel = self.get_acceleration(power_fraction);

        2.0 * (distance / accel).sqrt()
    }

    /// P / P_max = (a / a_max) ^ N
    fn get_energy_required(&self, distance: Length, power_fraction: Fraction) -> Energy {
        2.0 * self.max_power_draw
            * power_fraction.powf(Self::N_INVERSE)
            * (distance / self.max_acceleration).sqrt()
    }

    /// If a is proportional to P^(2/3), halving the trip duration requires 4x energy, 8x power
    fn get_acceleration(&self, power_fraction: Fraction) -> Acceleration {
        power_fraction.powf(Self::N_INVERSE) * self.max_acceleration
    }

    /// E = P * t
    fn get_power_fraction(&self, distance: Length, max_energy: Energy) -> Fraction {
        let intermediate =
            (max_energy / 2.0 / self.max_power_draw) / (distance / self.max_acceleration).sqrt();

        intermediate.powf(Self::N).into()
    }

    /// Get the power required to complete the trip in the given duration.
    /// Error is the fastest possible
    pub fn get_trip_details(
        &self,
        distance: Length,
        power_available: Power,
        energy_available: Energy,
    ) -> TripDetails {
        let pf = Fraction::clamp(power_available / self.max_power_draw);

        let duration_by_power = self.get_trip_duration(distance, pf);

        let duration_by_energy = self.get_trip_duration_from_energy(distance, energy_available);

        if duration_by_power > duration_by_energy {
            let energy_required = self.get_energy_required(distance, pf);

            TripDetails {
                duration: duration_by_power,
                power_required: power_available,
                energy_required,
            }
        } else {
            let pf = self.get_power_fraction(distance, energy_available);
            let power_required = pf.value() * self.max_power_draw;

            TripDetails {
                duration: duration_by_energy,
                power_required,
                energy_required: energy_available,
            }
        }
    }

    fn get_acceleration_from_duration(&self, distance: Length, duration: Duration) -> Acceleration {
        2.0 * distance / duration.squared()
    }

    fn get_trip_duration_from_energy(&self, distance: Length, energy: Energy) -> Duration {
        let power_fraction = self.get_power_fraction(distance, energy);
        self.get_trip_duration(distance, power_fraction)
    }

    /// Calculate the power required to sustain a given acceleration
    fn get_power(&self, accel_fraction: Fraction) -> Power {
        accel_fraction.powf(Self::N) * self.max_power_draw
    }

    // P / P_max = (a / a_max) ^ N
    const N: f64 = 1.5;
    const N_INVERSE: f64 = 1.0 / Self::N;
}

#[derive(Debug, Eq, PartialEq)]
pub struct TripDetails {
    pub duration: Duration,
    pub power_required: Power,
    pub energy_required: Energy,
}

#[cfg(test)]
fn example_warp_drive() -> WarpDrive {
    WarpDrive::new(Acceleration::in_m_per_s2(2.0), Power::in_watts(3.0))
}

#[test]
fn get_energy_required() {
    let drive = example_warp_drive();
    let distance = Length::in_m(7.0);
    let f_p = Fraction::clamp(0.3);

    // E = P * t
    let expected = f_p.value() * drive.max_power_draw * drive.get_trip_duration(distance, f_p);

    let energy = drive.get_energy_required(distance, f_p);

    assert!((energy - expected).value.abs() <= 0.000000000001);
}

#[test]
#[ignore]
fn get_trip_details() {
    let drive = example_warp_drive();
    let power = Power::in_watts(3.0);
    let energy = Energy::in_joules(4.0);

    for d in (1..10).into_iter().map(|i| i as f64).map(Length::in_m) {
        let trip = drive.get_trip_details(d, power, energy);
        println!("{} m: {:#?}", d.value(), trip);
    }

    panic!();
}

/// Fold spacetime to bring two distant points together. Powered by an energy discharge.
#[derive(Debug, Copy, Clone)]
pub struct JumpDrive {
    pub max_energy: Energy,
    pub charge: Energy,
    pub max_range: Length,
}
