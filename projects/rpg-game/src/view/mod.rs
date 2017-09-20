use calcium_rendering::{Renderer};
use calcium_rendering_simple2d::{RenderBatch};

use model::{Tiles};

pub struct TilesRenderer {
}

impl TilesRenderer {
    pub fn new() -> Self {
        TilesRenderer {
        }
    }

    pub fn render<R: Renderer>(&self, _tiles: &Tiles, _batches: &mut Vec<RenderBatch<R>>) {
    }
}
