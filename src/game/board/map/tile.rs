mod tile_basic;
mod tile_action;
use crate::common::entity::{Terrian, Placement};

#[derive(Clone)]
pub struct Tile {
    terrian: Terrian,
    placement: Placement,
    supply: bool,
}

