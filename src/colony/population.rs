use super::*;
use Resource::Food;

#[derive(Debug, Default)]
pub struct People {
    pub population: Component<Colony, Population>,
    pub satiation: Component<Colony, Satiation>,
}

impl People {
    pub fn insert<I: ValidId<Colony>>(&mut self, id: I, population: Population) {
        self.population.insert(id, population);
        self.satiation.insert(id, Satiation::new(1.0));
    }

    pub fn request_food(&mut self, resources: &mut Resources) {
        self.population
            .iter()
            .zip(resources.requested.get_mut(Food).iter_mut())
            .for_each(|(population, food_requested)| {
                *food_requested += population.get_food_requirement();
            });
    }

    pub fn take_food(&mut self, resources: &mut Resources) {
        let fulfillment = resources.fulfillment.get(Food).iter();
        let stockpile = resources.stockpile.get_mut(Food).iter_mut();

        self.population
            .iter()
            .zip(self.satiation.iter_mut())
            .zip(fulfillment)
            .zip(stockpile)
            .for_each(|(((population, satiation), fulfillment), food_stockpile)| {
                *food_stockpile -= population.get_food_requirement() * fulfillment * INTERVAL;
                satiation.add_next(*fulfillment);
            });
    }
}

const INTERVAL: DurationFloat = crate::systems::System::ColonyProductionCycle.get_interval_float();

impl Colonies {
    /// Sums the population on each body so that multiple colonies on the same body
    /// will have the effect of crowding each other out
    pub fn update_population(&mut self, bodies: &mut Bodies) {
        bodies.sum_population(self);

        let body_pop = self.body.iter().map(|b| bodies.population.get(b));
        let land_area = self.body.iter().map(|b| bodies.get_land_area(*b));

        self.people
            .population
            .iter_mut()
            .zip(self.people.satiation.iter())
            .zip(body_pop)
            .zip(land_area)
            .for_each(|(((pop, satiation), body_pop), land_area)| {
                let body_pop = body_pop.copied().unwrap_or(*pop);
                *pop *= get_population_multiplier(*satiation, land_area, body_pop);
            });
    }
}

// Logistic function:   dN/dt = r * N
//                      dN/dt = r_max * (K - N) / K * N
//
// where:               N = population
//                      r = growth rate (zero growth = 1.0)
// where:               K = N_max * r_max / (r_max - 1)
//                      N_max = ρ_max * surface area * land fraction * habitable fraction
//                      land fraction = land area / total area
//                      habitable fraction = habitable area / land area
//                      ρ_max = 12 billion / 104 million sq km
//
//                      land usage: https://ourworldindata.org/land-use
fn get_population_multiplier(
    satiation: Satiation,
    land_area: Area,
    body_population: Population,
) -> f64 {
    let max_pop = land_area * MAX_POPULATION_DENSITY;
    let k = max_pop * (BASE_GROWTH_MULTIPLIER / BASE_GROWTH_RATE);

    let mut k_factor = 1.0 - (body_population / k);
    k_factor = k_factor.max(0.01);

    let annual_growth_rate = BASE_GROWTH_MULTIPLIER * k_factor * satiation.value();

    annual_growth_rate.powf(YEAR_FRACTION)
}

const YEAR_FRACTION: f64 = System::ColonyPopulation.get_interval_as_year_fraction();
const BASE_GROWTH_RATE: f64 = 0.025;
const BASE_GROWTH_MULTIPLIER: f64 = 1.0 + BASE_GROWTH_RATE;

/// 12 billion / 104e6 sq km
const MAX_POPULATION_DENSITY: PopulationDensity =
    PopulationDensity::in_people_per_square_km(12e9 / 104e6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_population_multiplier_satiation() {
        let a = get_population_multiplier(
            Satiation::new(1.0),
            Area::in_square_km(1.0),
            Population::in_millions(0.001),
        );
        let b = get_population_multiplier(
            Satiation::new(0.9),
            Area::in_square_km(1.0),
            Population::in_millions(0.001),
        );

        assert!(a > b);
    }

    #[test]
    fn get_population_multiplier_area() {
        let a = get_population_multiplier(
            Satiation::new(1.0),
            Area::in_square_km(1.0),
            Population::in_millions(0.001),
        );
        let b = get_population_multiplier(
            Satiation::new(1.0),
            Area::in_square_km(0.5),
            Population::in_millions(0.001),
        );

        assert!(a > b);
    }

    #[test]
    fn get_population_multiplier_population() {
        let a = get_population_multiplier(
            Satiation::new(1.0),
            Area::in_square_km(1.0),
            Population::in_millions(0.001),
        );
        let b = get_population_multiplier(
            Satiation::new(1.0),
            Area::in_square_km(1.0),
            Population::in_millions(0.002),
        );

        assert!(a > b);
    }
}
