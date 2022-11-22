pub mod map;
mod board_new;
mod board_show;
mod board_king;

use map::Map;

#[derive(Clone)]
pub struct Board {
    map: Map,
    turn: i64,
    cp: i64,
}