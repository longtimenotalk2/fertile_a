use super::Board;

impl Board {
    pub(super) fn sow_all(&mut self) {
        self.map.sow_all();
    }
}