mod terrian;
mod placement;
mod natural;
mod manmade;
mod resource;

#[derive(Clone)]
pub enum Resource {
    Food,
    Wood,
}

#[derive(Clone)]
pub enum Terrian {
    Plain,
    Sea,
    Hill,
}

#[derive(Clone)]
pub enum Manmade {
    Hovel,
    Sawmill,
}

#[derive(Clone)]
pub enum Natural {
    Tree,
    Farm,
}

#[derive(Clone)]
pub enum Placement {
    Void,
    Landform(Natural),
    Building(Manmade),
    Foundation(Manmade, i64),
}