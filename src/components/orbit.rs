use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Orbit {
    pub params: OrbitParams,
    pub parent: Option<OrbitParams>,
}

impl Orbit {
    pub fn calculate_position(&self, time: TimeFloat) -> Position {
        if let Some(parent) = self.parent {
            parent.calculate_position(time) + self.params.calculate_position(time)
        } else {
            self.params.calculate_position(time)
        }
        .into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OrbitParams {
    pub radius: Length,
    pub period: DurationFloat,
    pub offset: Angle,
}

impl OrbitParams {
    pub fn calculate_position(&self, time: TimeFloat) -> Distance {
        let angle = self.get_angle(time);
        Distance::from_angle_and_radius(angle, self.radius)
    }

    pub fn get_angle(&self, time: TimeFloat) -> Angle {
        Angle::in_rad(time / self.period * Self::NEG_TWO_PI) - self.offset
    }

    const NEG_TWO_PI: f64 = std::f64::consts::PI * -2.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn orbit_test_at_time_zero() {
        let orbit = get_planet_orbit();
        let time = TimeFloat::in_s(0.0);

        let position = orbit.calculate_position(time);

        assert_eq!(Position::in_m(0.0, 1000.0), position);
    }

    #[test]
    fn orbit_test_at_quarter_orbit() {
        let orbit = get_planet_orbit();
        let time = TimeFloat::in_s(orbit.params.period.value / 4.0);

        let quarter = orbit.calculate_position(time);
        
        assert_eq!(Length::in_m(-1000.0), quarter.x);
        assert!(nearly_zero(quarter.y));
    }

    #[test]
    fn orbit_test_at_zero_with_quarter_offset() {
        let orbit = get_planet_orbit_with_offset(Angle::in_rad(PI / 2.0));
        let time = TimeFloat::in_s(0.0);

        let quarter = orbit.calculate_position(time);

        assert_eq!(Length::in_m(-1000.0), quarter.x);
        assert!(nearly_zero(quarter.y));
    }

    #[test]
    fn moon_orbit_test() {
        let orbit = get_moon_orbit();
        let time = TimeFloat::in_s(0.0);

        let moon_position = orbit.calculate_position(time);

        assert_eq!(Position::in_m(0.0, 1010.0), moon_position);
    }

    fn get_planet_orbit_with_offset(offset: Angle) -> Orbit {
        Orbit {
            params: OrbitParams {
                radius: Length::in_m(1000.0),
                period: DurationFloat::in_s(60.0),
                offset,
            },
            parent: None,
        }
    }

    fn get_planet_orbit() -> Orbit {
        get_planet_orbit_with_offset(Angle::default())
    }

    fn get_moon_orbit() -> Orbit {
        Orbit {
            params: OrbitParams {
                radius: Length::in_m(10.0),
                period: DurationFloat::in_s(10.0),
                offset: Angle::default(),
            },
            parent: Some(get_planet_orbit().params),
        }
    }

    fn nearly_zero(value: Length) -> bool {
        Length::in_m(0.00001) > value && value > Length::in_m(-0.00001)
    }
}