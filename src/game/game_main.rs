use super::Game;
use std::io;

impl Game {
    fn main_loop(&mut self) {
        loop {
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).expect("fail to read line");
            self.parse_cmd(&cmd);
        }
    }

    pub fn main(&mut self) {
        self.show();
        self.main_loop();
    }
}