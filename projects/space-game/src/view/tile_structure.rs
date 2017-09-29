use std::sync::{Arc};

use cgmath::{Vector2, Vector4, Point2, EuclideanSpace};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error};
use calcium_rendering_2d::render_data::{RenderData, RenderBatch, ShaderMode, Rectangle, RenderSet, Projection, Camera, UvMode};

use model::{TileStructure};

pub struct TileStructureView<R: RendererRaw> {
    texture: Arc<Texture<R>>,
    tileset_tiles_amount: Vector2<u32>,
    tileset_uv_per_tile: Vector2<f32>,
}

impl<R: RendererRaw> TileStructureView<R> {
    pub fn new(renderer: &mut Renderer<R>) -> Result<Self, Error> {
        let texture = Texture::new()
            .from_file("./assets/tiles.png")
            .with_nearest_sampling()
            .build(renderer)?;

        let texture_size = texture.size();
        let tile_size = 32;
        let tileset_tiles_amount = Vector2::new(
            texture_size.x as u32 / tile_size,
            texture_size.y as u32 / tile_size,
        );
        let tileset_uv_per_tile = Vector2::new(
            1.0 / tileset_tiles_amount.x as f32,
            1.0 / tileset_tiles_amount.y as f32
        );

        Ok(TileStructureView {
            texture,
            tileset_tiles_amount,
            tileset_uv_per_tile,
        })
    }

    pub fn render(
        &self,
        structure: &TileStructure, render_data: &mut RenderData<R>
    ) {
        let mut tiles_batch = RenderBatch::new(
            ShaderMode::Texture(self.texture.clone()), UvMode::YUp
        );

        let tile = 0;
        let source_position: Point2<f32> = Point2::new(
            tile % self.tileset_tiles_amount.x,
            tile / self.tileset_tiles_amount.x,
        ).cast();

        // Render the tiles
        for y in 0..structure.size().y {
            for x in 0..structure.size().x {
                let tile_position = Point2::new(x, y).cast();

                if !structure.tile_at(tile_position).unwrap().has_floor() {
                    continue
                }

                let tile_offset = tile_position.to_vec().cast();
                // I'm not sure why this needs to be subtracted from the V only
                let half_pixel = 1.0 / (8.0*32.0) / 2.0;
                tiles_batch.push_rectangle(
                    Rectangle::new(
                        Point2::new(0.0, 0.0) + tile_offset,
                        Point2::new(1.0, 1.0) + tile_offset,
                    ),
                    Rectangle::new(
                        Point2::new(
                            source_position.x * self.tileset_uv_per_tile.x,
                            (source_position.y + 1.0) * self.tileset_uv_per_tile.y - half_pixel,
                        ),
                        Point2::new(
                            (source_position.x + 1.0) * self.tileset_uv_per_tile.x,
                            source_position.y * self.tileset_uv_per_tile.y - half_pixel,
                        ),
                    ),
                    Vector4::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        }

        // Submit the rendering set
        let camera = Camera::new(64.0, Point2::new(50.0, 50.0));
        let background_set = RenderSet::new(Projection::Camera(camera), vec!(tiles_batch));
        render_data.render_sets.push(background_set);
    }
}
