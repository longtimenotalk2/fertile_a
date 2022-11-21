pub mod resource;
pub mod terrian;
pub mod placement;

use resource::Resource;
use terrian::Terrian;
use placement::Placement;


#[derive(Clone)]
pub struct Tile {
    terrian: Terrian,
    placement: Placement,
    supply: bool,
}

impl Tile {
    pub fn new() -> Self {
        Self {
            terrian: Terrian::Plain,
            placement: Placement::Void,
            supply: true,
        }
    }

    pub fn get_terrian(&self) -> &Terrian {
        &self.terrian
    }

    pub fn get_placement(&self) -> &Placement {
        &self.placement
    }

    pub(super) fn set_terrian(&mut self, terrian: Terrian) {
        self.terrian = terrian;
    }

    pub(super) fn set_placement(&mut self, placement: Placement) {
        self.placement = placement;
    }



}