use crate::star::Star;
use crate::body::Body;
use crate::colony::Colony;

#[derive(Debug, Default)]
pub struct State {
    pub star: Star,
    pub body: Body,
    pub colony: Colony,
}