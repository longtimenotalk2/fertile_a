mod terrian;
mod placement;
mod natural;
mod manmade;
mod resource;

#[derive(Clone, Debug)]
pub enum Resource {
    Food,
    Wood,
}

#[derive(Clone, Debug)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

#[derive(Clone, Debug)]
pub enum Manmade {
    Hovel,
    Sawmill,
}

#[derive(Clone, Debug)]
pub enum Natural {
    Tree,
    Farm,
}

#[derive(Clone, Debug)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}