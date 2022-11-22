pub mod tile;

mod map_basic;
mod map_new;
mod map_show;
mod map_find;


use tile::Tile;

#[derive(Clone)]
pub struct Map {
    n_row: i64,
    n_col: i64,
    tiles: Vec<Tile>,
}