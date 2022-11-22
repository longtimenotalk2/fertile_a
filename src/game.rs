pub mod board;
mod game_basic;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

