use super::*;
use crate::body::examples::{earth, luna};
use crate::body::Planet;
use crate::colony::economy::ProductionUnit;
use crate::colony::ColonyLinks;
use crate::star::examples::sol;
use crate::star::StarSystem;
use crate::system_state::SystemState;

struct TestState {
    state: SystemState,
    farm_colony: Id<Colony>,
    city_colony: Id<Colony>,
}

fn get_test_state() -> TestState {
    let mut state = SystemState::default();

    let (_sol, _bodies) = state.state.create(get_test_star_system());

    let earth = state.state.body.get_by_name("Earth").unwrap();
    let luna = state.state.body.get_by_name("Luna").unwrap();

    let earth_pop = Population::in_millions(100.0);
    let luna_pop = Population::in_millions(10.0);

    let farm_colony = state.state.colony.create(
        Colony {
            name: "Farmland Earth".to_string(),
            population: earth_pop,
        },
        ColonyLinks { body: earth },
    );

    // create production
    let food_required = ProductionUnit::new((earth_pop + luna_pop).get_food_requirement());
    let food_production = state.state.colony.production.get_mut(Facility::Farmland);
    food_production.insert(farm_colony, food_required);

    state.state.colony.production_cycle();

    let city_colony = state.state.colony.create(
        Colony {
            name: "Lunar City".to_string(),
            population: luna_pop,
        },
        ColonyLinks { body: luna },
    );

    // create price gradient
    let food_prices = state.state.colony.resources.prices.get_mut(Resource::Food);
    food_prices.insert(city_colony, Resource::Food.get_default_price() * 4.0);

    TestState {
        state,
        farm_colony,
        city_colony,
    }
}

fn get_test_star_system() -> StarSystem {
    StarSystem {
        star: sol(),
        planets: vec![Planet {
            body: earth(),
            moons: vec![luna()],
        }],
    }
}

fn get_random_freighter(rng: &mut impl rand::Rng) -> Freighter {
    let tonnage = rng.gen_range(100.0, 250.0) * TON;
    Freighter {
        tonnage,
        capacity: tonnage * 2.5,
        loading_rate: rng.gen_range(1.0, 2.5) * TON / MIN,
        drive: Drive::Warp(rng.gen_range(10.0, 30.0) * KM / S),
    }
}

#[allow(unused_variables)]
#[test]
fn idle_freighter_without_assignment_remains_idle() {
    let TestState {
        mut state,
        city_colony,
        farm_colony,
    } = get_test_state();

    let rng = &mut rand::thread_rng();

    for _ in 0..150 {
        let f = state.state.freighter.create(
            get_random_freighter(rng),
            FreighterLinks {
                location: farm_colony,
            },
        );

        {
            let f = Valid::assert(f);
            let assignment = &mut state.state.freighter.assignment;
            assignment.insert(f, Assignment::Route(city_colony, farm_colony));
        }
    }

    state.update_by(60.0 * MIN * 2000.0);

    let start = std::time::Instant::now();
    state.update_by(60.0 * MIN);
    let end = std::time::Instant::now();
    dbg!(end - start);

    println!("{:?}:\tdone", state.state.time.get_time());

    let get_name = |id: Id<Colony>| state.state.colony.name.get(id);
    let get_satiation = |id: Id<Colony>| state.state.colony.people.satiation.get(id).value();

    println!("{}: {}", get_name(city_colony), get_satiation(city_colony));
    println!("{}: {}", get_name(farm_colony), get_satiation(farm_colony));

    // panic!();
}
