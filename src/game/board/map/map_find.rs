use super::Map;
use crate::common::incorporeal::{Pos, Dir};

impl Map {
    pub fn valid(&self, pos: &Pos) -> bool {
        let (r, c) = pos.get();
        if r >= 0 && r < self.n_row && c >= 0 && c < self.n_col {
            true
        } else {
            false
        }
    }

    pub fn find_all(&self) -> Vec<Pos> {
        let mut result: Vec<Pos> = vec![];
        for row in 0..self.n_row {
            for col in 0..self.n_col {
                result.push(Pos::new(row, col));
            }
        }
        result
    }

    pub fn find_dir(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        if self.valid(pos) {
            let (row, col) = pos.get();
            match dir {
                Dir::R => {
                    if col < self.n_col - 1 {
                        Some(Pos::new(row, col + 1))
                    } else {
                        None
                    }
                }
                Dir::DR => {
                    if row < self.n_row - 1 && (row % 2 == 0 || col < self.n_col - 1) {
                        Some(Pos::new(row + 1, col + row % 2))
                    } else {
                        None
                    }
                }
                Dir::DL => {
                    if row < self.n_row - 1 && (row % 2 == 1 || col > 0) {
                        Some(Pos::new(row + 1, col - (1 - row % 2)))
                    } else {
                        None
                    }
                }
                Dir::L => {
                    if col > 0 {
                        Some(Pos::new(row, col - 1))
                    } else {
                        None
                    }
                }
                Dir::UL => {
                    if row > 0 && (row % 2 == 1 || col > 0) {
                        Some(Pos::new(row - 1, col - (1 - row % 2)))
                    } else {
                        None
                    }
                }
                Dir::UR => {
                    if row > 0 && (row % 2 == 0 || col < self.n_col - 1) {
                        Some(Pos::new(row - 1, col + row % 2))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn find_adjs(&self, pos: &Pos) -> Vec<Pos> {
        if self.valid(pos) {
            [Dir::R, Dir::DR, Dir::DL, Dir::L, Dir::UL, Dir::UR]
                .iter()
                .filter_map(|d| self.find_dir(pos, d))
                .collect()
        } else {
            vec![]
        }
    }

}