use super::Game;
use crate::common::{incorporeal::Dir, entity::Manmade, reason::*};

const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[m";

fn refuse(txt: &str) {
    println!("{}Refuse : {}{}", RED, txt, RESET);
}

impl Game {
    fn update(&mut self) {
        let board_new = self.board().clone();
        self.boards.push(board_new);
    }

    pub fn cmd_undo(&mut self) {
        if self.boards.len() > 1 {
            self.boards.pop();
            self.show();
        } else {
            refuse("Initial state, can not undo")
        }
    }

    pub(super) fn cmd_end(&mut self) {
        self.update();
        self.board_mut().king_end();
        self.show();
    }

    pub fn cmd_move(&mut self, dir: &Dir) {
        if let Err(reason) = self.board().king_mvcost(dir) {
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_move(dir).unwrap();
        self.show();
    }

    pub fn cmd_pick(&mut self) {
        if let Err(reason) = self.board().king_can_pick() {
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_pick().unwrap();
        self.show();
    }

    pub(super) fn cmd_found(&mut self, manmade: Manmade) {
        if let Err(reason) = self.board().king_can_found(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_found(manmade).unwrap();
        self.show();
    }

    pub(super) fn cmd_build(&mut self)  {
        if let Err(reason) = self.board().king_can_build(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_build().unwrap();
        self.show();
    }

    pub(super) fn cmd_build_to_finish(&mut self) {
        if let Err(reason) = self.board().king_can_build(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_build().unwrap();
        while let Ok(_) = self.board().king_can_build(){
            self.board_mut().king_build().unwrap();
        }
        self.show();
    }

    pub(super) fn cmd_saw(&mut self) {
        if let Err(reason) = self.board().king_can_saw(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_saw().unwrap();
        self.show();
    }

    pub(super) fn cmd_sow(&mut self) {
        if let Err(reason) = self.board().king_can_sow(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_sow().unwrap();
        self.show();
    }

    pub(super) fn cmd_ruin(&mut self) {
        if let Err(reason) = self.board().king_can_ruin(){
            refuse(&reason.str());
            return;
        }
        self.update();
        self.board_mut().king_ruin().unwrap();
        self.show();
    }

    pub(super) fn cmd_work(&mut self) {
        if let Err(Reason::ActOnWrongPlacement(..)) = self.board().king_can_pick() {
            if let Err(Reason::ActOnWrongPlacement(..)) = self.board().king_can_saw() {
                if let Err(Reason::ActOnWrongPlacement(_, p)) = self.board().king_can_build() {
                    refuse(&Reason::ActOnWrongPlacement(Action::Work, p.clone()).str());
                }else{
                    self.cmd_build();
                }
            }else{
                self.cmd_saw();
            }
        }else{
            self.cmd_pick();
        }
    }

    pub(super) fn cmd_invalid(&self) {
        refuse("Invalid input");
    }
}