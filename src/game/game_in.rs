use crate::common::{incorporeal::Dir, entity::Manmade};
use super::Game;

impl Game {
    pub fn parse_cmd(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        match cmd {
            "s" => self.cmd_move(&Dir::R),
            "x" => self.cmd_move(&Dir::DR),
            "z" => self.cmd_move(&Dir::DL),
            "a" => self.cmd_move(&Dir::L),
            "q" => self.cmd_move(&Dir::UL),
            "w" => self.cmd_move(&Dir::UR),
            "fh" => self.cmd_found(Manmade::Hovel),
            "fs" => self.cmd_found(Manmade::Sawmill),
            "b" => self.cmd_build_to_finish(),
            "e" => self.cmd_end(),
            "u" => self.cmd_undo(),
            "p" => self.cmd_work(),
            _ => {
                if let Some(num) = cmd.strip_prefix("e").and_then(|s| s.parse::<i64>().ok()) {
                    for _ in 0..num {
                        self.cmd_end();
                    }
                }else{
                    self.cmd_invalid()
                }
            },
        };
    }

}
