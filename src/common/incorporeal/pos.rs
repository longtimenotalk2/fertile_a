use super::Pos;

impl Pos {
    pub fn new(r: i64, c: i64) -> Self {
        Self { r, c }
    }

    pub fn from_usize(u : usize, n_col : i64) -> Self {
        Self {
            r : u as i64 / n_col,
            c : u as i64 % n_col,
        }
    }

    pub fn into_usize(&self, n_col: i64) -> usize {
        let i = self.r * n_col + self.c;
        i.try_into().unwrap()
    }
    
    pub fn get(&self) -> (i64, i64) {
        (self.r, self.c)
    }

    pub fn str(&self) -> String {
        format!("({}, {})", self.r, self.c)
    }
}