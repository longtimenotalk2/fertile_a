use crate::common::entity::*;

pub enum Action {
    Move,
    Found,
    Build,
    Sow,
    Pick,
    Saw,
}

pub enum Reason {
    ActOnWrongTerrian(Action, Terrian),
    ActOnWrongPlacement(Action, Placement),
    Consumed,
    LackInventory(Resource),
}