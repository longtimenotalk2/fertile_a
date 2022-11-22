pub mod game;
pub mod common;

use game::Game;

fn main() {
    let mut mygame = Game::new();
    mygame.show();
}