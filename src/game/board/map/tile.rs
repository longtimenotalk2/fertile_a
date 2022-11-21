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