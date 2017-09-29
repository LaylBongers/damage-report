use std::sync::{Arc};

use cgmath::{Vector2, Vector4, Point2};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error};
use calcium_rendering_2d::render_data::{RenderData, RenderBatch, ShaderMode, DrawRectangle, Rectangle, RenderSet, Projection, Camera};

use model::{TileStructure};

pub struct TileStructureView<R: RendererRaw> {
    texture: Arc<Texture<R>>,
}

impl<R: RendererRaw> TileStructureView<R> {
    pub fn new(renderer: &mut Renderer<R>) -> Result<Self, Error> {
        let texture = Texture::new()
            .from_file("./assets/tiles.png")
            .with_linear_sampling()
            .build(renderer)?;

        Ok(TileStructureView {
            texture,
        })
    }

    pub fn render(
        &self,
        structure: &TileStructure, render_data: &mut RenderData<R>, renderer: &mut Renderer<R>
    ) {
        let mut tiles_batch = RenderBatch::new(
            ShaderMode::Texture(self.texture.clone())
        );

        for y in 0..structure.size().y {
            for x in 0..structure.size().x {
                let offset = Vector2::new(x, y).cast();

                tiles_batch.push_rectangle(DrawRectangle {
                    destination: Rectangle::new(
                            Point2::new(0.0, 0.0) + offset,
                            Point2::new(1.0, 1.0) + offset,
                        ),
                    texture_source: None,
                    color: Vector4::new(1.0, 1.0, 1.0, 1.0),
                });
            }
        }

        // Submit the rendering set
        let camera = Camera::new(32.0, Point2::new(0.0, 0.0));
        let background_set = RenderSet::new(Projection::Camera(camera), vec!(tiles_batch));
        render_data.render_sets.push(background_set);
    }
}
