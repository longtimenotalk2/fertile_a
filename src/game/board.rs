pub mod map;
use map::Map;

#[derive(Clone)]
pub struct Board {
    map: Map,
    turn: i64,
    cp: i64,
}