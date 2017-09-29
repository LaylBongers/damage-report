use cgmath::{Vector2, Point2};
use rand::{Rng};

use model::{Tile};

#[derive(Debug, Clone)]
pub struct TileStructure {
    tiles: Vec<Tile>,
    size: Vector2<u32>,
}

impl TileStructure {
    pub fn empty(size: Vector2<u32>) -> Self {
        TileStructure {
            tiles: vec![Tile::empty(); size.x as usize * size.y as usize],
            size,
        }
    }

    pub fn randomize_floors(&mut self) {
        let mut rng = ::rand::StdRng::new().unwrap();
        for tile in &mut self.tiles {
            tile.set_floor(rng.gen())
        }
    }

    pub fn tile_at(&self, position: Point2<i32>) -> Option<&Tile> {
        if position.x < 0 || position.x >= self.size.x as i32 ||
           position.y < 0 || position.y >= self.size.y as i32 {
            None
        } else {
            let tile = &self.tiles[(position.x + (position.y * self.size.x as i32)) as usize];
            Some(tile)
        }
    }
}
