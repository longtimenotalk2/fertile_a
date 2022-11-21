pub mod board;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

