use super::Map;
use crate::common::entity::*;

impl Map {
    pub fn refresh_all(&mut self) {
        for pos in self.find_all() {
            self.tile_mut(&pos).refresh();
        }
    }

    pub fn sow_all(&mut self) {
        for pos in self.find_placement(Placement::Building(Manmade::Hovel)) {
            if let Ok(_) = self.tile_mut(&pos).sow() {
                ()
            }else{
                ()
            }
        }
    }
}