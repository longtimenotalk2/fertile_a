use super::Placement;
use super::Manmade;
use crate::common::reason::Reason;

impl Placement {
    pub fn str(&self) -> String {
        match self {
            Placement::Void => "Nothing".to_string(),
            Placement::Landform(n) => format!("{} as Landform", n.str()),
            Placement::Building(m) => format!("{} as Building", m.str()),
            Placement::Foundation(m, p) => format!("{} as Foundation", m.str()),
        }
    }

    pub fn get_building(&self) -> Option<Manmade> {
        if let Placement::Building(m) = self {
            Some(m.clone())
        }else{
            None
        }
    }

    pub fn may_step(&self) -> Result<f64, Reason> {
        if let Placement::Landform(n) = self {
            n.may_step()
        }else{
            Ok(0.)
        }
    }

    
    

}