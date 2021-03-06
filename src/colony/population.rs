use super::*;
use Resource::Food;

type Satiation = ExpMovingAvg<f64, 15.0>;

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
        let population = self.population.iter();
        let requested = resources.demand.get_mut(Food).iter_mut();

        population
            .zip(requested)
            .for_each(|(population, requested)| {
                *requested += population.get_food_requirement();
            });
    }

    pub fn take_food(&mut self, resources: &mut Resources) {
        let food_required = self.population.iter().map(|pop| pop.get_food_requirement());
        let satiation = self.satiation.iter_mut();
        let fulfillment = resources.fulfillment.get(Food).iter();
        let stockpile = resources.stockpile.get_mut(Food).iter_mut();

        let iter = food_required.zip(satiation).zip(fulfillment).zip(stockpile);

        for (((food_required, satiation), fulfillment), food_stockpile) in iter {
            *food_stockpile -= food_required * fulfillment * INTERVAL;
            satiation.add_next(*fulfillment);
        }
    }
}

const INTERVAL: Duration = crate::systems::System::ColonyProductionCycle.get_interval();

impl Colonies {
    /// Sums the population on each body so that multiple colonies on the same body
    /// will have the effect of crowding each other out
    // TODO area should be a colony component, remove body population
    pub fn update_population(&mut self, bodies: &mut Bodies) {
        bodies.sum_population(self);

        let population = self.people.population.iter_mut();
        let satiation = self.people.satiation.iter();
        let body = self.body.iter();

        for ((pop, satiation), body) in population.zip(satiation).zip(body) {
            let body_pop = bodies.population.get(body).copied().unwrap_or(*pop);

            let land_area = bodies.get_land_area(body);

            *pop *= get_population_multiplier(*satiation, land_area, body_pop);
        }
    }
}

// Logistic function:   dN/dt = r * N
//                      dN/dt = r_max * (K - N) / K * N
//
// where:               N = population
//                      r = growth rate (zero growth = 1.0)
//                      K = N_max * r_max / (r_max - 1)
//                      r_max = ρ_max * surface area * land fraction * habitable fraction
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
    const YEAR_FRACTION: f64 = System::ColonyPopulation.get_interval_as_year_fraction();

    let max_pop = land_area * MAX_POPULATION_DENSITY;
    let k = max_pop * (BASE_GROWTH_MULTIPLIER / BASE_GROWTH_RATE);

    let mut k_factor = 1.0 - (body_population / k);
    k_factor = k_factor.max(0.01);

    let annual_growth_rate = BASE_GROWTH_MULTIPLIER * k_factor * satiation.value();

    annual_growth_rate.powf(YEAR_FRACTION)
}

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
