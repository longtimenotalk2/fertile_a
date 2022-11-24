use super::Map;
use crate::common::{reason::*, incorporeal::*, entity::*};

impl Map {
    pub fn mvcost(&self, pos: &Pos, dir: &Dir) -> Result<f64, Reason> {
        match self.find_dir(pos, dir) {
            None => Err(Reason::ActOutOfBoundary(Action::Move)),
            Some(p) => self.tile(&p).mvcost(),
        }
    }

    pub fn can_found(&self, pos: &Pos) -> Result<(), Reason> {
        self.tile(pos).can_found()
    }

    pub fn found(&mut self, pos: &Pos, manmade: Manmade) -> Result<(), Reason> {
        self.tile_mut(pos).found(manmade)
    }

    pub fn can_build(&self, pos: &Pos) -> Result<(), Reason> {
        self.tile(pos).can_build()
    }

    pub fn build(&mut self, pos: &Pos) -> Result<bool, Reason> {
        self.tile_mut(pos).build()
    }

    pub fn can_pick(&self, pos: &Pos) -> Result<Resource, Reason> {
        self.tile(pos).can_pick()
    }

    pub fn pick(&mut self, pos: &Pos) -> Result<Resource, Reason> {
        self.tile_mut(pos).pick()
    }

    pub fn can_saw(&self, pos : &Pos) -> Result<Pos, Reason> {
        self.tile(pos).may_saw()?;
        let mut result : Option<Result<Resource, Reason>> = None;
        for p in self.find_adjs(pos) {
            if let Some(Resource::Wood) = self.tile(&p).get_resource() {
                result = Some(self.tile(&p).can_pick());
                if let Some(Ok(_)) = result {
                    return Ok(p);
                }
            }
        }
        if let Some(rslt) = result {
            if let Err(_) = rslt {
                Err(Reason::ActAdjConsumed(Action::Saw, Resource::Wood))
            }else{
                unreachable!();
            }
        }else{
            Err(Reason::ActNeedAdjPlacement(Action::Saw, Placement::Landform(Natural::Tree)))
        }
    }

    pub fn saw(&mut self, pos : &Pos) -> Result<(), Reason> {
        let p = self.can_saw(pos)?;
        self.tile_mut(&p).pick().map(|_| ()).unwrap();
        Ok(())
    }
    
    pub fn can_sow(&self, pos : &Pos) -> Result<(), Reason> {
        self.tile(pos).can_sow()
    }

    pub fn sow(&mut self, pos : &Pos) -> Result<(), Reason> {
        self.tile_mut(pos).sow()
    }

    pub fn can_give_sow(&self, pos : &Pos) -> Result<Pos, Reason> {
        for p in self.find_adjs(pos) {
            if let Ok(_) = self.can_sow(&p) {
                return Ok(p);
            }
        }
        Err(Reason::ActNeedAdjPlacement(Action::Sow, Placement::Void))
    }

    pub fn give_sow(&mut self, pos : &Pos) -> Result<(), Reason> { 
        let target = self.can_give_sow(pos)?;
        self.sow(&target)
    }

    pub fn can_ruin(&self, pos : &Pos) -> Result<i64, Reason>  {
        self.tile(pos).can_ruin()
    }

    pub fn ruin(&mut self, pos : &Pos) -> Result<i64, Reason> {
        self.tile_mut(pos).ruin()
    }

}