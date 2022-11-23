mod terrian;
mod placement;
mod natural;
mod manmade;
mod resource;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resource {
    Food,
    Wood,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Manmade {
    Hovel,
    Sawmill,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Natural {
    Tree,
    Farm,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}