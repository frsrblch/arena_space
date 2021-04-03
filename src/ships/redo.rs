#![allow(dead_code, unused_variables)]

use crate::star::Star;
use crate::*;
use std::collections::VecDeque;

pub type Comp<T> = Component<Ship, T>;

#[derive(Debug, Clone)]
pub struct Ship;

dynamic_arena! { Ship }

#[derive(Debug, Clone)]
pub struct Ships {
    pub alloc: Allocator<Ship>,

    pub fleet: Comp<Fleet>,

    pub crew: Comp<Crew>,
    pub reactor: Comp<Reactor>,
    pub sensor: Comp<Option<Sensor>>,
    pub signature: Comp<Signature>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Crew {
    pub crew: Population,
    pub experience: Fraction,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Reactor {
    pub max_heat: Power,
    pub waste_heat: Fraction,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Signature {
    pub emissions: Power,
    pub size: Area,
}

#[derive(Debug, Default, Clone)]
pub struct Sensor {
    pub threshold: Power,
}

type FComp<T> = Component<Fleet, T>;

#[derive(Debug, Clone)]
pub struct Fleet;

dynamic_arena!(Fleet);

#[derive(Debug, Default, Clone)]
pub struct Fleets {
    pub alloc: Allocator<Fleet>,
    pub data: FleetData,
}

#[derive(Debug, Default, Clone)]
pub struct FleetData {
    pub name: FComp<String>,
    pub ships: FComp<HashSet<Id<Ship>>>,

    pub star: IdLink<Fleet, Star>,
    pub position: FComp<Position>,

    pub signature: FComp<Signature>,

    pub contacts: FComp<Vec<Contact>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Contact {
    pub ship: Id<Fleet>,
    pub bearing: Angle,
}

pub struct Navigation {
    navigation: FComp<NavigationState>,
    navigation_queue: IndexedMinQueue<Fleet, TimeFloat>,
}

impl Navigation {}

pub struct NavigationState {
    pub current: NavigationAction,
    pub queue: VecDeque<NavigationAction>,
}

pub struct NavigationAction {
    pub action_type: NavigationStateEnum,
    pub duration: Option<Duration>,
}

impl NavigationAction {
    pub fn duration(&self, action: &Fleets, params: &StateParameters) -> Duration {
        todo!()
    }

    pub fn get_cost(&self, action: &Fleets, params: &StateParameters) -> f64 {
        todo!()
    }
}

pub enum NavigationStateEnum {
    Drift(Position, Velocity),
    Orbit(Id<Body>),
    FtlTo(Id<Star>),
    AccelTowardBody(Id<Body>),
}

pub struct StateParameters<'a> {
    pub stars: &'a Stars,
    pub bodies: &'a Bodies,
    pub colonies: &'a Colonies,
    pub fleets: &'a mut Fleets,
}
