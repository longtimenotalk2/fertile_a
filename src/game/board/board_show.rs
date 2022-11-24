use crate::common::constant::CP_MAX;

use super::Board;

impl Board {
    fn print_turn(&self) {
        println!(
            "Turn : {} (CP : {}/{})",
            self.turn,
            self.cp,
            CP_MAX,
        );
    }

    fn print_king_info(&self) {
        println!(
            "Inventory : food = {}, wood = {}",
            self.king.get_food(),
            self.king.get_wood()
        )
    }

    pub fn show_map_only(&self) {
        self.map.show_map_only();
    }

    pub fn show_map_with_king(&self){
        self.map.show_map_with_king(self.king.get_pos());
    }

    pub fn show_default(&self) {
        self.show_map_with_king();
        self.print_turn();
        self.print_king_info();
    }
}