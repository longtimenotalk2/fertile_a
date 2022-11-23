use super::{Placement, Manmade, Natural};
use crate::common::reason::{Action, Reason};

impl Placement {
    pub fn str(&self) -> String {
        match self {
            Placement::Void => "Nothing".to_string(),
            Placement::Landform(n) => format!("{} as Landform", n.str()),
            Placement::Building(m) => format!("{} as Building", m.str()),
            Placement::Foundation(m, _) => format!("{} as Foundation", m.str()),
        }
    }

    pub fn get_building(&self) -> Option<Manmade> {
        if let Placement::Building(m) = self {
            Some(m.clone())
        }else{
            None
        }
    }

    pub fn mvcost(&self) -> Result<f64, Reason> {
        if let Placement::Landform(n) = self {
            n.mvcost()
        }else{
            Ok(0.)
        }
    }

    pub fn may_found(&self) -> Result<(), Reason> {
        match self {
            Placement::Void => Ok(()),
            Placement::Landform(natural) => match natural {
                Natural::Tree => Err(Reason::ActOnWrongPlacement(Action::Found, self.clone())),
                Natural::Farm => Ok(()),
            }
            _ => Err(Reason::ActOnWrongPlacement(Action::Found, self.clone())),
        }
    }

    pub fn may_sow(&self) -> Result<(), Reason> {
        match self {
            Placement::Void => Ok(()),
            _ => Err(Reason::ActOnWrongPlacement(Action::Sow, self.clone()))
        }
    }

    pub fn may_build(&self) -> Result<(), Reason> {
        if let Placement::Foundation(..) = self {
            Ok(())
        }else{
            Err(Reason::ActOnWrongPlacement(Action::Build, self.clone()))
        }
    }

    
    

}