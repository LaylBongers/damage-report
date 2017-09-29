use std::sync::{Arc};

use cgmath::{Point2, EuclideanSpace};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error};
use calcium_rendering_2d::render_data::{RenderData, RenderBatch, ShaderMode, DrawRectangle, Rectangle, RenderSet, Projection, UvMode};

pub struct BackgroundView<R: RendererRaw> {
    texture: Arc<Texture<R>>,
}

impl<R: RendererRaw> BackgroundView<R> {
    pub fn new(renderer: &mut Renderer<R>) -> Result<Self, Error> {
        let texture = Texture::new()
            .from_file("./assets/background.jpg")
            .with_linear_sampling()
            .build(renderer)?;

        Ok(BackgroundView {
            texture
        })
    }

    pub fn render(&self, render_data: &mut RenderData<R>, renderer: &mut Renderer<R>) {
        let mut background_batch = RenderBatch::new(
            ShaderMode::Texture(self.texture.clone()), UvMode::YDown
        );

        let source_size = self.texture.size().cast();
        let target_size = renderer.size().cast();
        let half_target_size = target_size * 0.5;

        // If this needs to be scaled up, scale it up
        let mut rect_size = source_size;
        if target_size.x > rect_size.x {
            rect_size = rect_size * (target_size.x / source_size.x);
        }
        if target_size.y > rect_size.y {
            rect_size = rect_size * (target_size.y / source_size.y);
        }

        background_batch.push_rectangle(DrawRectangle::full_texture(Rectangle::new(
            Point2::from_vec(half_target_size - rect_size * 0.5),
            Point2::from_vec(half_target_size + rect_size * 0.5),
        )));

        let background_set = RenderSet::new(Projection::Pixels, vec!(background_batch));
        render_data.render_sets.push(background_set);
    }
}
