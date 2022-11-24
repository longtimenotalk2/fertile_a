use super::Map;
use super::tile::Tile;
use crate::common::entity::{Placement, Manmade, Resource};
use crate::common::incorporeal::Pos;


impl Map {
    pub(super) fn tile(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    pub(super) fn tile_mut(&mut self, pos: &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }

    pub fn get_resource(&self, pos : &Pos) -> Option<Resource> {
        self.tile(pos).get_resource()
    }

    pub fn get_power(&self, pos : &Pos) -> Option<i64> {
        if let Placement::Building(Manmade::Hovel) = self.tile(pos).get_placement() {
            let mut power = 1;
            for p in self.find_adjs(pos) {
                if let Some(Resource::Food) = self.get_resource(&p) {
                    power += 1;
                }
            }
            Some(power)
        }else{
            None
        }
    }
}