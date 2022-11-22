use super::Map;
use crate::common::entity::*;
use crate::common::incorporeal::Pos;

// https://fuhao.xiao84.com/86855.html
// https://crates.io/crates/colorful

const RED: &str = "\u{1b}[31m";
const GREEN: &str = "\u{1b}[32m";
const UGREEN: &str = "\u{1b}[4;32m";
const YELLOW: &str = "\u{1b}[33m";
const UYELLOW: &str = "\u{1b}[4;33m";
const BLUE: &str = "\u{1b}[34m";
const RESET: &str = "\u{1b}[m";

pub enum Content<'a> {
    Default(&'a Pos),
}

pub struct ShowStyle<'a> {
    content : Content<'a>,
    coordinate : bool,
}

impl Map {
    fn print_tile(&self, pos: &Pos) {
        let tile = self.tile(&pos);
        match tile.get_terrian() {
            Terrian::Sea => print!("{}Sea{}", BLUE, RESET),
            _ => {
                // Left
                match tile.get_terrian() {
                    Terrian::Plain => print!(" "),
                    Terrian::Sea => (),
                    Terrian::Hill => print!("H"),
                }
                // Middle
                print!(" ");
                //Right
                match tile.get_placement() {
                    Placement::Void => print!(" "),
                    Placement::Landform(n) => match n {
                        Natural::Tree => {
                            if tile.get_supply() {
                                print!("{}T{}", GREEN, RESET)
                            } else {
                                print!("{}T{}", UGREEN, RESET)
                            }
                        }
                        Natural::Farm => {
                            if tile.get_supply() {
                                print!("{}f{}", YELLOW, RESET)
                            } else {
                                print!("{}f{}", UYELLOW, RESET)
                            }
                        }
                    },
                    Placement::Building(m) => match m {
                        Manmade::Hovel => print!("h"),
                        Manmade::Sawmill => print!("s"),
                        _ => print!(" "),
                    },
                    Placement::Foundation(m, process) => match m {
                        Manmade::Hovel => print!("{}h{}", RED, RESET),
                        Manmade::Sawmill => print!("{}s{}", RED, RESET),
                        _ => print!(" "),
                    },
                }
            }
        }
    }

    fn print_frame(&self) {
        // col num
        for c in 0..self.n_col {
            print!("   {}", c);
        }
        print!("\n");
        
        // first line
        print!(" "); // row num
        print!("┌───");
        for _col in 0..self.n_col - 1 {
            print!("┬───");
        }
        print!("┐\n");
        // others line
        for row in 0..self.n_row {
            //// 1line
            print!("{}", row); // row num
            // leftest block
            if row % 2 == 1 {
                print!("  ");
            }
            print!("│");
            // other block
            for col in 0..self.n_col {
                let pos = Pos::new(row, col);
                // TILE BLOCK
                self.print_tile(&pos);
                print!("│");
            }
            print!("\n");
            //// 2line
            print!(" "); // row num
            if row < self.n_row - 1 {
                if row % 2 == 0 {
                    print!("└");
                    for _col in 0..self.n_col {
                        print!("─┬─┴");
                    }
                    print!("─┐\n");
                } else {
                    print!("┌");
                    for _col in 0..self.n_col {
                        print!("─┴─┬");
                    }
                    print!("─┘\n");
                }
            }
        }
        // last line
        if self.n_row % 2 == 0 {
            print!("  ");
        }
        print!("└───");
        for _col in 0..self.n_col - 1 {
            print!("┴───");
        }
        print!("┘");
        print!("\n");
    }

    pub fn show_default(&self) {
        self.print_frame();
    }

}

