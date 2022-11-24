use super::Tile;
use crate::common::{reason::{Action, Reason}, entity::*};

impl Tile {
    pub fn mvcost(&self) -> Result<f64, Reason> {
        self.terrian.mvcost().and_then(|mv1| self.placement.mvcost().map(|mv2| mv1+mv2))
    }

    pub fn can_found(&self) -> Result<(), Reason> {
        self.terrian.may_found().and(
            self.placement.may_found()
        )
    }

    pub fn found(&mut self, manmade: Manmade) -> Result<(), Reason> {
        self.can_found()?;
        self.placement = Placement::Foundation(manmade, 0);
        Ok(())
    }

    pub fn can_build(&self) -> Result<(), Reason> {
        self.terrian.may_build().and(self.placement.may_build())
    }

    pub fn build(&mut self) -> Result<bool, Reason> {
        self.can_build()?;
        if let Placement::Foundation(m, p) = &self.placement {
            if p + 1 == m.get_max_process() {
                self.placement = Placement::Building(m.clone());
                Ok(true)
            }else{
                self.placement = Placement::Foundation(m.clone(), p+1);
                Ok(false)
            }
        }else{
            unreachable!();
        }
    }

    pub fn can_pick(&self) -> Result<Resource, Reason> {
        if let Some(r) = self.get_resource() {
            if self.supply {
                Ok(r)
            }else{
                Err(Reason::ActConsumed(Action::Pick, r))
            }
        }else{
            Err(Reason::ActOnWrongPlacement(Action::Pick, self.placement.clone()))
        }
    }

    pub fn pick(&mut self) -> Result<Resource, Reason> {
        let r = self.can_pick()?;
        self.supply = false;
        Ok(r)
    }

    pub fn may_saw(&self) -> Result<(), Reason> {
        if let Placement::Building(Manmade::Sawmill) = self.placement{
            Ok(())
        }else{
            Err(Reason::ActOnWrongPlacement(Action::Saw, self.placement.clone()))
        }
    }

    // pub fn may_sow(&self) -> Result<(), Reason> {
    //     if let Placement::Building(Manmade::Hovel) = self.placement{
    //         Ok(())
    //     }else{
    //         Err(Reason::ActOnWrongPlacement(Action::Sow, self.placement.clone()))
    //     }
    // }

    pub fn can_sow(&self) -> Result<(), Reason> {
        match self.terrian {
            Terrian::Sea => Err(Reason::ActOnWrongTerrian(Action::Sow, Terrian::Sea)),
            _ => match self.placement {
                Placement::Void => Ok(()),
                _ => Err(Reason::ActOnWrongPlacement(Action::Sow, self.placement.clone())),
            }
        }
    }

    pub fn sow(&mut self) -> Result<(), Reason> {
        self.can_sow()?;
        self.set_placement(Placement::Landform(Natural::Farm));
        Ok(())
    }

}