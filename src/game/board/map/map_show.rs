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
    Default,
    AA(&'a Pos),
}

pub struct ShowStyle<'a> {
    content : Content<'a>,
    coordinate : bool,
    king : Option<&'a Pos>,
    power : bool,
}

impl Map {
    fn print_middle(&self, pos: &Pos, style : &ShowStyle) {
        match style.content {
            Content::Default => {
                let mut char = " ".to_string();
                let mut use_color = true;
                let mut color = "".to_string();
                let mut is_king = false;
                if let Some(king_pos) = style.king {
                    is_king = pos.eq(king_pos);
                }

                let tile = self.tile(pos);
                //king
                if is_king {
                    char = "@".to_string();
                    use_color = false;
                }
                //process
                if let Some(process) = tile.get_process() {
                    char = format!("{}", process);
                    color = RED.to_string();
                }
                //power
                if style.power {
                    if let Some(power) = self.get_power(pos) {
                        char = format!("{}", power);
                        color = YELLOW.to_string();
                    }
                }

                if use_color {
                    print!("{}{}{}", color, char, RESET);
                }else{
                    print!("{}", char);
                }
            },
            _ => print!(" "),
        }
    }
    
    fn print_tile(&self, pos: &Pos, style : &ShowStyle) {
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
                self.print_middle(pos, style);
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
                    },
                    Placement::Foundation(m, _) => match m {
                        Manmade::Hovel => print!("{}h{}", RED, RESET),
                        Manmade::Sawmill => print!("{}s{}", RED, RESET),
                    },
                }
            }
        }
    }

    fn print_frame(&self, style : &ShowStyle) {
        // col num
        if style.coordinate {
            for c in 0..self.n_col {
                print!("   {}", c);
            }
            print!("\n");
        }
        
        // first line
        if style.coordinate {
            print!(" "); // row num
        }
        print!("┌───");
        for _col in 0..self.n_col - 1 {
            print!("┬───");
        }
        print!("┐\n");
        // others line
        for row in 0..self.n_row {
            //// 1line
            if style.coordinate {
                print!("{}", row); // row num
            }
            // leftest block
            if row % 2 == 1 {
                print!("  ");
            }
            print!("│");
            // other block
            for col in 0..self.n_col {
                let pos = Pos::new(row, col);
                // TILE BLOCK
                self.print_tile(&pos, &style);
                print!("│");
            }
            print!("\n");
            //// 2line
            if style.coordinate {
                print!(" "); // row num
            }
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

    fn show(&self, style : &ShowStyle) {
        self.print_frame(style);
    }

    pub fn show_map_only(&self) {
        let style = ShowStyle {
            content : Content::Default,
            coordinate : true,
            king : None,
            power : true,
        };
        self.show(&style);
    }
    
    pub fn show_map_with_king(&self, king_pos : &Pos) {
        let style = ShowStyle {
            content : Content::Default,
            coordinate : true,
            king : Some(king_pos),
            power : true,
        };
        self.show(&style);
    }
}

