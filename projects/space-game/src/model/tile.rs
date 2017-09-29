#[derive(Debug, Clone, Copy)]
pub struct Tile {
    has_floor: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            has_floor: false,
        }
    }

    pub fn has_floor(&self) -> bool {
        self.has_floor
    }

    pub fn set_floor(&mut self, value: bool) {
        self.has_floor = value;
    }
}
