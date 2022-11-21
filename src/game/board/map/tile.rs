pub mod entity;
use entity::Terrian;

#[derive(Clone)]
pub struct Tile {
    terrian: Terrian,
    // placement: Placement,
    supply: bool,
}