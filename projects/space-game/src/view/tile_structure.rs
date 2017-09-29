pub use std::sync::{Arc};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error};
use calcium_rendering_2d::render_data::{RenderData, RenderBatch, ShaderMode, DrawRectangle, Rectangle, RenderSet, Projection};

pub struct TileStructureView<R: RendererRaw> {
    texture: Arc<Texture<R>>,
}

impl<R: RendererRaw> TileStructureView<R> {
    pub fn new(renderer: &mut Renderer<R>) -> Result<Self, Error> {
        let texture = Texture::new()
            .from_file("./assets/tiles.jpg")
            .with_linear_sampling()
            .build(renderer)?;

        Ok(TileStructureView {
            texture,
        })
    }

    pub fn render(&self, render_data: &mut RenderData<R>, renderer: &mut Renderer<R>) {
        let mut background_batch = RenderBatch::new(
            ShaderMode::Texture(self.texture.clone())
        );
    }
}
