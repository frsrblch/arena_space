use crate::body::BodyProperties;
use crate::ships::freighter_state::FreighterStateIndex;
use crate::ships::Freighter;
use crate::star::StarType;
use crate::*;
use gen_id::{Id, Valid, ValidId};
use iter_context::ContextualIterator;

#[derive(Debug)]
pub struct EntityMut<'a, ID> {
    state: &'a mut State,
    pub id: Valid<'a, Id<ID>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Entity<'a, ID> {
    state: &'a State,
    pub id: Valid<'a, Id<ID>>,
}

impl<A> Entity<'_, A> {
    pub fn id(&self) -> Id<A> {
        self.id.id()
    }
}

impl State {
    pub fn get<A>(&self, id: impl ValidId<A>) -> Entity<A> {
        Entity {
            state: self,
            id: Valid::assert(id.id()),
        }
    }

    pub fn get_mut<A>(&mut self, id: impl ValidId<A>) -> EntityMut<A> {
        EntityMut {
            state: self,
            id: Valid::assert(id.id()),
        }
    }
}

impl<'a> Entity<'a, Star> {
    pub fn name(&self) -> &'a str {
        self.state.star.name.get(self.id)
    }

    pub fn bodies(&self) -> impl Iterator<Item = Entity<Body>> {
        self.state
            .star
            .bodies
            .get(self.id)
            .iter()
            .map(move |b| self.state.get(b))
    }

    pub fn position(&self) -> Position {
        *self.state.star.position.get(self.id)
    }

    pub fn radius(&self) -> Length {
        self.state.star.star_type.get(self.id).get_radius()
    }

    pub fn star_type(&self) -> StarType {
        *self.state.star.star_type.get(self.id)
    }
}

impl<'a> Entity<'a, Body> {
    pub fn name(&self) -> &'a str {
        self.state.body.name.get(self.id)
    }

    pub fn radius(&self) -> Length {
        *self.state.body.radius.get(self.id)
    }

    pub fn orbit(&self) -> &BodyOrbit {
        self.state.body.orbit.get(self.id)
    }

    pub fn position(&self) -> Position {
        self.orbit().calculate_position(self.state.time.get_time())
    }

    pub fn position_at_time(&self, time: TimeFloat) -> Position {
        self.orbit().calculate_position(time)
    }

    pub fn properties(&self) -> BodyProperties {
        *self.state.body.properties.get(self.id)
    }

    pub fn star(&self) -> Entity<'a, Star> {
        self.state.get(self.state.body.star.get(self.id))
    }

    pub fn colonies(&self) -> impl Iterator<Item = Entity<Colony>> {
        let body = self.state.colony.body.iter();
        let colony = self.state.colony.alloc.ids();

        body.zip(colony).into_iter().filter_map(move |(b, c)| {
            if *b == self.id.value {
                Some(self.state.get(c))
            } else {
                None
            }
        })
    }
}

impl<'a> Entity<'a, Colony> {
    pub fn name(&self) -> &'a str {
        self.state.colony.name.get(self.id)
    }

    pub fn body(&self) -> Entity<Body> {
        self.state.get(self.state.colony.body.get(self.id))
    }

    pub fn star(&self) -> Entity<Star> {
        self.body().star()
    }

    pub fn population(&self) -> Population {
        *self.state.colony.people.population.get(self.id)
    }

    pub fn price(&self, resource: Resource) -> Price {
        self.state.colony.resources.price.get(resource)[self.id]
    }

    pub fn supply(&self, resource: Resource) -> MassRate {
        self.state.colony.resources.supply.get(resource)[self.id]
    }

    pub fn demand(&self, resource: Resource) -> MassRate {
        self.state.colony.resources.demand.get(resource)[self.id]
    }

    pub fn stockpile(&self, resource: Resource) -> Mass {
        self.state.colony.resources.stockpile.get(resource)[self.id]
    }
}

impl<'a> Entity<'a, Freighter> {
    pub fn name(&self) -> &'a str {
        self.state.freighter.name.get(self.id)
    }

    pub fn position(&self) -> Position {
        let state = &self.state.freighter.state;
        match self.state.freighter.state.indices().get(self.id) {
            FreighterStateIndex::Idle(index) => {
                let colony = state.idle.location.get(index);
                self.state.get(colony).body().position()
            }
            FreighterStateIndex::Loading(index) => {
                let colony = state.loading.location.get(index);
                self.state.get(colony).body().position()
            }
            FreighterStateIndex::Unloading(index) => {
                let colony = state.unloading.location.get(index);
                self.state.get(colony).body().position()
            }
            FreighterStateIndex::Moving(index) => {
                let source = state.moving.source.get(index);
                let destination = state.moving.destination.get(index);

                let departure = *state.moving.departure.get(index).unwrap();
                let arrival = *state.moving.arrival.get(index).unwrap();
                let time = self.state.time.get_time();

                let departure_pos = self.state.get(source).body().position_at_time(departure);
                let arrival_pos = self.state.get(destination).body().position_at_time(arrival);

                let fraction = Fraction::clamp((time - departure) / (arrival - departure));
                let trip_vector = arrival_pos - departure_pos;
                departure_pos + trip_vector * fraction.value()
            }
        }
    }
}
