pub mod board;
mod game_basic;
mod game_cmd;
mod game_in;
mod game_main;

use board::Board;

#[derive(Clone)]
pub struct Game {
    boards: Vec<Board>,
}

