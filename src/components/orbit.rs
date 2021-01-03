use super::*;

#[derive(Debug, Copy, Clone)]
pub struct BodyOrbit {
    pub params: Orbit,
    pub parent: Option<Orbit>,
}

impl BodyOrbit {
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
pub struct Orbit {
    pub radius: Length,
    pub angular_speed: AngularSpeed,
    pub offset: Angle,
}

impl Orbit {
    pub fn from_period(radius: Length, period: Duration, offset: Angle) -> Self {
        Self {
            radius,
            angular_speed: -Angle::TWO_PI / period,
            offset,
        }
    }

    pub fn calculate_position(&self, time: TimeFloat) -> Distance {
        let angle = self.get_angle(time);
        Distance::from_angle_and_radius(angle, self.radius)
    }

    pub fn get_angle(&self, time: TimeFloat) -> Angle {
        time.value * self.angular_speed - self.offset
    }

    pub fn calculate_speed(&self) -> Speed {
        Speed::in_m_per_s(self.radius.value() * self.angular_speed.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn get_planet_orbit() -> Orbit {
        get_planet_orbit_with_offset(Angle::default())
    }

    fn get_planet_orbit_with_offset(offset: Angle) -> Orbit {
        Orbit::from_period(Length::in_m(1000.0), Duration::in_s(60.0), offset)
    }

    #[test]
    fn orbit_test_at_time_zero() {
        let orbit = get_planet_orbit();
        let time = TimeFloat::in_s(0.0);

        let position = orbit.calculate_position(time);

        assert_eq!(Distance::in_m(0.0, 1000.0), position);
    }

    #[test]
    fn orbit_test_at_quarter_orbit() {
        let orbit = get_planet_orbit();
        let time = -Angle::TWO_PI / orbit.angular_speed / 4.0;
        assert!(time > Duration::zero());

        let quarter = orbit.calculate_position(TimeFloat::in_s(time.value()));

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

    fn get_moon_orbit() -> BodyOrbit {
        BodyOrbit {
            params: Orbit::from_period(Length::in_m(10.0), Duration::in_s(10.0), Angle::default()),
            parent: Some(get_planet_orbit()),
        }
    }

    fn nearly_zero(value: Length) -> bool {
        Length::in_m(0.00001) > value && value > Length::in_m(-0.00001)
    }
}
