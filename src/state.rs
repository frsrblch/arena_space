use crate::star::Stars;
use crate::body::Bodies;
use crate::colony::Colonies;
use crate::nation::Nations;
use crate::time::TimeState;
use crate::systems::Systems;

#[derive(Debug, Default)]
pub struct State {
    pub time: TimeState,
    pub systems: Systems,
    pub star: Stars,
    pub body: Bodies,
    pub nation: Nations,
    pub colony: Colonies,
}
