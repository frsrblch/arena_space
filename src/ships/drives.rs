use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Drive {
    Warp(Speed),
}

type TripDuration = DurationFloat;

impl Drive {
    pub fn calculate_trip_duration<I: ValidId<Colony>>(
        &self,
        from: I,
        to: I,
        departure: TimeFloat,
        colonies: &Colonies,
        bodies: &Bodies,
        stars: &Stars,
    ) -> TripDuration {
        let from_body = colonies.body.get(from);
        let from_star = bodies.star.get(from_body);

        let to_body = colonies.body.get(to);
        let to_star = bodies.star.get(to_body);

        if from_star == to_star {
            let from_orbit = bodies.orbit.get(from_body);
            let to_orbit = bodies.orbit.get(to_body);

            self.calculate_trip_duration_in_system(from_orbit, to_orbit, departure)
        } else {
            let from_position = stars.position.get(from_star);
            let to_position = stars.position.get(to_star);

            self.calculate_trip_duration_interstellar(*from_position, *to_position)
        }
    }

    fn calculate_trip_duration_in_system(
        &self,
        from_orbit: &Orbit,
        to_orbit: &Orbit,
        departure: TimeFloat,
    ) -> TripDuration {
        let from_position = from_orbit.calculate_position(departure);
        let mut duration = DurationFloat::zero();

        // Newton's method used to compensate for orbital motion
        for _ in 0..5 {
            let to_position = to_orbit.calculate_position(departure + duration);

            let distance = (from_position - to_position).magnitude();

            duration = match self {
                Drive::Warp(speed) => distance / speed,
            };
        }

        duration
    }

    fn calculate_trip_duration_interstellar(&self, from: Position, to: Position) -> TripDuration {
        let distance = (from - to).magnitude();

        match self {
            Drive::Warp(speed) => distance / speed,
        }
    }
}
