use crate::common::entity::*;

pub enum Action {
    Move,
    Found,
    Build,
    Sow,
}

pub enum Reason {
    ActOnWrongTerrian(Action, Terrian),
}