use super::Game;
use crate::common::incorporeal::Dir;

const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

impl Game {
    fn update(&mut self) {
        let board_new = self.board().clone();
        self.boards.push(board_new);
    }

    pub fn cmd_undo(&mut self) {
        if self.boards.len() > 1 {
            self.boards.pop();
            self.show();
        } else {
            refuse("Initial state, can not undo")
        }
    }

    // pub(super) fn cmd_move(&mut self, dir: &Dir) {
    //     if let Err(e) = self.board().king_mvcost(dir) {
    //         refuse_err(&e, "move");
    //         return;
    //     }
    //     self.update();
    //     self.board_mut().king_move(dir).expect("panic in king move");
    //     self.show();
    // }

}