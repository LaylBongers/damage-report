use cgmath::{Vector2, Point2};
use tiled::{Layer};

pub struct TilesLayer {
    size: Vector2<u32>,
    tiles: Vec<u32>,
}

impl TilesLayer {
    pub fn new(layer: &Layer, size: Vector2<u32>) -> Self {
        assert_eq!(layer.tiles.len(), size.y as usize);
        assert_eq!(layer.tiles[0].len(), size.x as usize);

        let mut tiles = Vec::with_capacity((size.x * size.y) as usize);

        for rows in &layer.tiles {
            for tile in rows {
                tiles.push(*tile);
            }
        }

        TilesLayer {
            tiles,
            size,
        }
    }

    pub fn tiles(&self) -> &Vec<u32> {
        &self.tiles
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn tile(&self, position: Point2<u32>) -> u32 {
        self.tiles[(position.x + position.y * self.size.x) as usize]
    }
}
