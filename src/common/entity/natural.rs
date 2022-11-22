use super::Natural;
use crate::common::{reason::Reason, constant::*};

impl Natural {
    pub fn str(&self) -> String {
        match self {
            Natural::Tree => "Tree".to_string(),
            Natural::Farm => "Farm".to_string(),
        }
    }

    pub(super) fn mvcost(&self) -> Result<f64, Reason> {
        match self {
            Natural::Tree => Ok(MVCOST_TREE),
            _ => Ok(0.),
        }
    }
}