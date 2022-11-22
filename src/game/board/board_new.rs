use super::Board;
use super::map::Map;

impl Board {
    pub fn new_example() -> Self {
        Self {
            map: Map::new_example(),
            turn: 1,
            cp: 0,
        }
    }
}
