use crate::common::entity::*;

#[derive(Debug)]
pub enum Action {
    Move,
    Found,
    Build,
    Sow,
    Pick,
    Saw,
}

#[derive(Debug)]
pub enum Reason {
    ActOnWrongTerrian(Action, Terrian),
    ActOnWrongPlacement(Action, Placement),
    ActConsumed(Action, Resource),
    ActAdjConsumed(Action, Resource),
    LackInventory(Resource),
    OutOfBoundary,
    ActNeedAdjPlacement(Action, Placement),
}