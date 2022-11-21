use super::Map;
use super::tile::Tile;
use crate::common::incorporeal::Pos;


impl Map {
    pub(super) fn tile(&self, pos: &Pos) -> &Tile {
        &self.tiles[pos.into_usize(self.n_col)]
    }

    pub(super) fn tile_mut(&mut self, pos: &Pos) -> &mut Tile {
        &mut self.tiles[pos.into_usize(self.n_col)]
    }
}