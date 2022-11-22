use super::Board;
use super::map::Map;

impl Board {
    pub fn new_std() -> Self {
        Self {
            map: Map::new_std(),
            turn: 1,
            cp: 0,
        }
    }
}
