use super::Manmade;
use crate::common::constant::*;

impl Manmade {
    pub fn str(&self) -> String {
        match self {
            Manmade::Hovel => "Hovel".to_string(),
            Manmade::Sawmill => "Sawmill".to_string(),
        }
    }
    
    pub fn get_max_process(&self) -> i64 {
        match self {
            Manmade::Hovel => PROCESS_HOVEL,
            Manmade::Sawmill => PROCESS_SAWMILL,
        }
    }
}