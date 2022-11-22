use super::Board;
use crate::common::constant::*;

impl Board {
    fn start_turn(&mut self) {
        self.turn += 1;
        self.map.refresh_all();
    }

    fn end_turn(&mut self) {
        if self.turn % 4 == 0 {
            self.sow_all();
        }
    }

    pub fn pass_cp(&mut self, cp: f64) {
        self.cp += cp;
        while self.cp >= MAX_CP {
            self.cp -= MAX_CP;
            self.end_turn();
            self.start_turn();
        }
    }
}