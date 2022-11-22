use super::Dir;

impl Dir {
    pub fn anti(&self) -> Dir {
        match self {
            Dir::R => Dir::L,
            Dir::DR => Dir::UL,
            Dir::DL => Dir::UR,
            Dir::L => Dir::R,
            Dir::UL => Dir::DR,
            Dir::UR => Dir::DL,
        }
    }
}