use super::Board;
use crate::common::{reason::Reason, incorporeal::*, entity::Resource};

#[derive(Clone)]
pub(super) struct King {
    pos: Pos,
    food: i64,
    wood: i64,
}

impl King {
    pub(super) fn new(pos: Pos) -> Self {
        Self {
            pos,
            food: 10,
            wood: 5,
        }
    }

    pub fn get_pos(&self) -> &Pos {
        &self.pos
    }

    pub fn get_food(&self) -> i64 {
        self.food
    }

    pub fn get_wood(&self) -> i64 {
        self.wood
    }

    fn use_food(&mut self) -> Result<(), Reason> {
        if self.food > 0 {
            self.food -= 1;
            Ok(())
        } else {
            Err(Reason::LackInventory(Resource::Food))
        }
    }

    fn use_wood(&mut self) -> Result<(), Reason> {
        if self.wood > 0 {
            self.wood -= 1;
            Ok(())
        } else {
            Err(Reason::LackInventory(Resource::Wood))
        }
    }

    fn set_pos(&mut self, pos: &Pos) {
        self.pos = pos.clone();
    }
}

