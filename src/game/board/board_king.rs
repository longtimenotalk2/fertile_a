use super::Board;
use crate::common::{reason::Reason, incorporeal::*, entity::*, constant::*};

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

impl Board {
    pub fn king_end(&mut self) {
        let cp = CP_MAX - self.cp;
        self.pass_cp(cp);
    }

    pub fn king_mvcost(&self, dir: &Dir) -> Result<f64, Reason> {
        self.map.mvcost(self.king.get_pos(), dir)
    }

    pub fn king_move(&mut self, dir: &Dir) -> Result<(), Reason> {
        let mvcost = self.map.mvcost(self.king.get_pos(), dir)?;
        // move
        let p = self.map.find_dir(self.king.get_pos(), dir).unwrap();
        self.king.set_pos(&p);
        //cpcost
        self.pass_cp(mvcost);
        Ok(())
    }

    pub fn king_can_pick(&self) -> Result<Resource, Reason> {
        self.map.can_pick(&self.king.get_pos())
    }

    pub fn king_pick(&mut self) -> Result<(), Reason> {
        //pick
        let r = self.map.pick(&self.king.get_pos())?;
        match r {
            Resource::Food => self.king.food += 1,
            Resource::Wood => self.king.wood += 1,
        }
        //cpcost
        self.pass_cp(CP_BASE);
        Ok(())
    }

    pub fn king_can_found(&self) -> Result<(), Reason> {
        self.map.can_found(self.king.get_pos())
    }

    pub fn king_found(&mut self, manmade: Manmade) -> Result<(), Reason> {
        //found
        self.map.found(&self.king.get_pos(), manmade)?;
        //cpcost
        self.pass_cp(CP_BASE);
        Ok(())
    }

    pub fn king_can_build(&self) -> Result<(), Reason> {
        self.map.can_build(&self.king.get_pos())?;
        if self.king.get_wood() > 0 {
            if self.king.get_food() > 0 {
                Ok(())
            }else{
                Err(Reason::LackInventory(Resource::Food))
            }
        } else {
            Err(Reason::LackInventory(Resource::Wood))
        }
    }

    pub fn king_build(&mut self) -> Result<bool, Reason> {
        self.king_can_build()?;
        self.king.use_food().unwrap();
        self.king.use_wood().unwrap();
        let result = self.map.build(&self.king.get_pos()).unwrap();
        self.pass_cp(CP_BASE);
        Ok(result)
    }

    pub fn king_can_saw(&self) -> Result<(), Reason> {
        self.map.can_saw(self.king.get_pos())?;
        if self.king.get_food() > 0 {
            Ok(())
        }else{
            Err(Reason::LackInventory(Resource::Food))
        }
    }

    pub fn king_saw(&mut self) -> Result<(), Reason> {
        self.king_can_saw()?;
        self.map.saw(self.king.get_pos()).unwrap();
        self.king.use_food().unwrap();
        self.king.wood += 1;
        Ok(())
    }

}

