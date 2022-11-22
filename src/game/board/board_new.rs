use super::Board;
use super::map::Map;
use super::board_king::King;
use crate::common::incorporeal::Pos;

impl Board {
    pub fn new_example() -> Self {
        Self {
            map: Map::new_example(),
            turn: 1,
            cp: 0.,
            king: King::new(Pos::new(1,1)),
        }
    }
}
