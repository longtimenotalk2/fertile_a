mod pos;
mod dir;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    r: i64,
    c: i64,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    R,
    DR,
    DL,
    L,
    UL,
    UR,
}

