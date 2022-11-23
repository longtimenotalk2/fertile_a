use super::Board;

impl Board {
    pub fn show_map_only(&self) {
        self.map.show_map_only();
    }

    pub fn show_map_with_king(&self){
        self.map.show_map_with_king(self.king.get_pos());
    }
}