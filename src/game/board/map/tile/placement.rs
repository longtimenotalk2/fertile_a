pub mod natural;
pub mod manmade;

use natural::Natural;
use manmade::Manmade;

#[derive(Clone)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}