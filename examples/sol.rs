use arena_space::colony::economy::ProductionUnit;
use arena_space::colony::{Colony, ColonyLinks};
use arena_space::components::{Facility, MassRate, Population};
use arena_space::nation::Nation;
use arena_space::star::examples::sol_system;
use arena_space::system_state::SystemState;
use arena_space::time::get_date;

fn main() {
    let mut system_state = setup();

    system_state.update(get_date(2051, 1, 1));
}

fn setup() -> SystemState {
    let mut ss = SystemState::default();

    let (_sol, _) = ss.state.create(sol_system());

    let earth = ss.state.body.get_by_name("Earth").unwrap();

    let humanity = ss.state.nation.create(Nation {
        name: "The Imperium".to_string(),
    });

    let earth_population = Population::in_millions(5.75e3);
    let earth_colony = ss.state.colony.create(
        Colony {
            name: "Terra".to_string(),
            population: earth_population,
        },
        ColonyLinks {
            body: earth,
            nation: Some(humanity),
        },
    );

    ss.state
        .colony
        .production
        .get_mut(Facility::Farmland)
        .insert_unvalidated(
            earth_colony,
            ProductionUnit::new(earth_population.get_food_requirement() * 0.25),
        );

    let ore = MassRate::in_tons_per_day(5_475_701.0);
    ss.state
        .colony
        .production
        .get_mut(Facility::Mine)
        .insert_unvalidated(earth_colony, ProductionUnit::new(ore));

    let multiplier = 1.1 / Facility::Foundry.get_inputs().first().unwrap().multiplier;
    let metal = ore * multiplier;
    ss.state
        .colony
        .production
        .get_mut(Facility::Foundry)
        .insert_unvalidated(earth_colony, ProductionUnit::new(metal));

    ss
}
