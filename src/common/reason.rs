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

impl Action {
    fn str(&self) -> &str {
        match self {
            Move => "move",
            Found => "found",
            Build => "build",
            Sow => "sow",
            Pick => "pick",
            Saw => "saw",
        }
    }
}

#[derive(Debug)]
pub enum Reason {
    ActOnWrongTerrian(Action, Terrian),
    ActOnWrongPlacement(Action, Placement),
    ActConsumed(Action, Resource),
    ActAdjConsumed(Action, Resource),
    ActLackInventory(Action, Resource),
    ActOutOfBoundary(Action),
    ActNeedAdjPlacement(Action, Placement),
}

impl Reason {
    pub fn str(&self) -> String {
        match self {
            Reason::ActOnWrongTerrian(a, t) => format!{"Can not {} in {}", a.str(), t.str()},
            Reason::ActOnWrongPlacement(a, p) => format!{"Can not {} in tile with {}", a.str(), p.str()},
            Reason::ActConsumed(a, r) => format!{"Can not {} because {} is consumed", a.str(), r.str()},
            Reason::ActAdjConsumed(a, r) => format!{"Can not {} because adjacent {} are consumed", a.str(), r.str()},
            Reason::ActLackInventory(a, r) => format!{"Can not {}, lack {} in the inventory", a.str(), r.str()},
            Reason::ActOutOfBoundary(a) => format!{"Can not {}, out of boundary", a.str()},
            Reason::ActNeedAdjPlacement(a, p) => format!{"Can not {}, need {} around", a.str(), p.str()},
        }
    }
}