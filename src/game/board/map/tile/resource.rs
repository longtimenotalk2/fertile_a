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


