use super::Map;
use super::tile::Tile;
use super::pos::Pos;
use crate::common::entity::{Terrian, Placement, Natural, Manmade};

impl Map {
    pub fn new(n_row: i64, n_col: i64) -> Self {
        Self {
            n_row,
            n_col,
            tiles: vec![Tile::new(); (n_row * n_col).try_into().unwrap()],
        }
    }

    fn manually_set_terrian(&mut self, r: i64, c: i64, terrian: Terrian) {
        self.tile_mut(&Pos::new(r, c)).set_terrian(terrian);
    }

    fn manually_set_landform(&mut self, r: i64, c: i64, natural: Natural) {
        self.tile_mut(&Pos::new(r, c))
            .set_placement(Placement::Landform(natural));
    }

    fn manually_set_building(&mut self, r: i64, c: i64, manmade: Manmade) {
        self.tile_mut(&Pos::new(r, c))
            .set_placement(Placement::Building(manmade));
    }

    pub fn new_std() -> Self {
        let mut m = Map::new(6, 8);
        m.manually_set_terrian(2, 2, Terrian::Sea);
        m.manually_set_terrian(0, 3, Terrian::Hill);
        m.manually_set_landform(0, 2, Natural::Tree);
        m.manually_set_landform(1, 5, Natural::Tree);
        m.manually_set_landform(2, 5, Natural::Tree);
        m.manually_set_landform(3, 5, Natural::Tree);
        m.manually_set_building(1, 1, Manmade::Hovel);
        m.manually_set_building(1, 2, Manmade::Sawmill);

        m
    }

}