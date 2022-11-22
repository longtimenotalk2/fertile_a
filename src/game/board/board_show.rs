use super::Board;

impl Board {
    pub fn show_map_only(&self) {
        self.map.show_default();
    }
}