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

fn test_freighter() -> Freighter {
    Freighter {
        tonnage: 200.0 * TON,
        capacity: 500.0 * TON,
        loading_rate: 2.2 * TON / MIN,
        drive: Drive::Warp(21.0 * KM / S),
    }
}

// const m: Length = Length::in_m(1.0);
const KM: Length = Length::in_m(1e3);
// const kg: Mass = Mass::in_kg(1.0);
const TON: Mass = Mass::in_kg(1e3);
const S: DurationFloat = DurationFloat::in_s(1.0);
const MIN: DurationFloat = DurationFloat::in_s(60.0);
// const hr: DurationFloat = DurationFloat::in_hours(1.0);

#[allow(unused_variables)]
#[test]
fn idle_freighter_without_assignment_remains_idle() {
    let TestState {
        mut state,
        city_colony,
        farm_colony,
    } = get_test_state();

    let f = state.state.freighter.create(
        test_freighter(),
        FreighterLinks {
            location: farm_colony,
        },
    );

    {
        let f = Valid::assert(f);
        let assignment = &mut state.state.freighter.assignment;
        assignment.insert(f, Assignment::Route(city_colony, farm_colony));
    }

    let get_idle_len = |state: &SystemState| state.state.freighter.state.idle.len();
    let get_loading_len = |state: &SystemState| state.state.freighter.state.loading.len();
    let get_moving_len = |state: &SystemState| state.state.freighter.state.moving.len();
    let get_unloading_len = |state: &SystemState| state.state.freighter.state.unloading.len();

    let is_idle = |state: &SystemState| get_idle_len(state) == 1;
    let is_loading = |state: &SystemState| get_loading_len(state) == 1;
    let is_moving = |state: &SystemState| get_moving_len(state) == 1;
    let is_unloading = |state: &SystemState| get_unloading_len(state) == 1;

    while is_idle(&state) {
        state.update_by(1.0 * MIN);
    }

    match state.state.freighter.state.indices().get(Valid::assert(f)) {
        FreighterStateIndex::Loading(_) => {}
        st => panic!("{:?}", st),
    }

    // TODO ensure that fractional updates are not ignored

    let start = std::time::Instant::now();
    // for _ in 0..(24 * 10 * 60) {
    state.update_by(60.0 * MIN * 24.0 * 10.0);
    // }
    let end = std::time::Instant::now();
    dbg!(end - start);

    println!("{:?}:\tdone", state.state.time.get_time());

    panic!();
}
