pub mod board;
mod game_basic;
mod game_cmd;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

