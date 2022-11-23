use super::Tile;
use crate::common::{entity::*};

impl Tile {
    pub fn new() -> Self {
        Self {
            terrian: Terrian::Plain,
            placement: Placement::Void,
            supply: true,
        }
    }

    pub fn get_terrian(&self) -> &Terrian {
        &self.terrian
    }

    pub fn get_placement(&self) -> &Placement {
        &self.placement
    }

    pub fn set_terrian(&mut self, terrian: Terrian) {
        self.terrian = terrian;
    }

    pub fn set_placement(&mut self, placement: Placement) {
        self.placement = placement;
    }

    pub fn get_supply(&self) -> bool {
        self.supply
    }

    pub fn get_building(&self) -> Option<Manmade> {
        self.placement.get_building()
    }

    pub fn refresh(&mut self) {
        self.supply = true;
    }

    pub fn get_resource(&self) -> Option<Resource> {
        if let Placement::Landform(n) = &self.placement {
            match n {
                Natural::Farm => Some(Resource::Food),
                Natural::Tree => Some(Resource::Wood),
            }
        }else{
            None
        }
    }

    pub fn get_process(&self) -> Option<i64> {
        if let Placement::Foundation(_, p) = &self.placement {
            Some(*p)
        } else {
            None
        }
    }
}