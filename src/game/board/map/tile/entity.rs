use crate::constant::*;
use crate::reason::Reason;

#[derive(Clone)]
pub enum Resource {
    Food,
    Wood,
}

impl Resource {
    pub fn str(&self) -> String {
        match self {
            Resource::Food => "Food".to_string(),
            Resource::Wood => "Wood".to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

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