pub mod tile;
pub mod pos;

mod map_basic;
mod map_new;
mod map_show;


use tile::Tile;

#[derive(Clone)]
pub struct Map {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
}