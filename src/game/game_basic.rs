use super::Game;
use super::board::Board;

impl Game {
    pub fn board_mut(&mut self) -> &mut Board {
        self.boards.last_mut().unwrap()
    }

    pub fn board(&self) -> &Board {
        self.boards.last().unwrap()
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            boards: vec![Board::new_example()],
        }
    }
    pub fn show(&self) {
        self.board().show_default();
    }
}