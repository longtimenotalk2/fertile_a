use super::Terrian;
use crate::common::{reason::{Action, Reason}, constant::*};

impl Terrian {
    pub fn str(&self) -> String {
        match self {
            Terrian::Plain => "Plain".to_string(),
            Terrian::Sea => "Sea".to_string(),
            Terrian::Hill => "Hill".to_string(),
        }
    }

    pub fn mvcost(&self) -> Result<f64, Reason> {
        match self {
            Terrian::Plain => Ok(MVCOST_PLAIN),
            Terrian::Hill => Ok(MVCOST_HILL),
            Terrian::Sea => Err(Reason::ActOnWrongTerrian(Action::Move, Terrian::Sea)),
        }
    }
    
    pub fn may_found(&self) -> Result<(), Reason> {
        match self {
            Terrian::Sea => Err(Reason::ActOnWrongTerrian(Action::Found, Terrian::Sea)),
            _ => Ok(()),
        }
    }

    pub fn may_build(&self) -> Result<(), Reason> {
        match self {
            Terrian::Sea => Err(Reason::ActOnWrongTerrian(Action::Build, Terrian::Sea)),
            _ => Ok(()),
        }
    }

    pub fn may_sow(&self) -> Result<(), Reason> {
        match self {
            Terrian::Sea => Err(Reason::ActOnWrongTerrian(Action::Sow, Terrian::Sea)),
            _ => Ok(()),
        }
    }
}