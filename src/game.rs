pub mod board;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn board_mut(&mut self) -> &mut Board {
        self.boards.last_mut().unwrap()
    }

    fn board(&self) -> &Board {
        self.boards.last().unwrap()
    }
}

