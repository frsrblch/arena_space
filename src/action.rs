use crate::colony::ColonyLinks;
use crate::*;

pub enum ColonyActions {
    Body(ColonyBodyAction),
}

pub enum ColonyBodyAction {
    StartColony,
}

impl ColonyBodyAction {
    pub fn execute<C>(self, _colony: C, body: Id<Body>, state: &mut State)
    where
        C: ValidId<Colony>,
    {
        match self {
            ColonyBodyAction::StartColony => {
                let new_colony = Colony {
                    name: "New Colony".to_string(),
                    population: Population::zero(),
                };

                let links = ColonyLinks { body };

                state.colony.create(new_colony, links);
            }
        }
    }
}
