pub mod map;
mod board_new;
mod board_show;
mod board_king;
mod board_turn;
mod board_global;

use map::Map;
use board_king::King;

#[derive(Clone)]
pub struct Board {
    map: Map,
    turn: i64,
    cp: f64,
    king : King
}