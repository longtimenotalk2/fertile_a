use super::Terrian;
use crate::common::{reason::Reason, constant::*};

impl Terrian {
    pub fn str(&self) -> String {
        match self {
            Terrian::Plain => "Plain".to_string(),
            Terrian::Sea => "Sea".to_string(),
            Terrian::Hill => "Hill".to_string(),
        }
    }

    pub(super) fn mvcost(&self) -> Result<f64, Reason> {
        match self {
            Terrian::Plain => Ok(MVCOST_PLAIN),
            Terrian::Hill => Ok(MVCOST_HILL),
            Terrian::Sea => Err(Reason::Terrian(Terrian::Sea)),
        }
    }

    
}