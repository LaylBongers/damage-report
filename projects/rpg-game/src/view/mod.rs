use std::sync::{Arc};
use std::path::{PathBuf};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};
use calcium_rendering::{Renderer, Error};
use calcium_rendering_simple2d::render_data::{RenderBatch, ShaderMode, DrawRectangle, Rectangle};
use cgmath::{Vector2, Point2};
use tiled::{Map as TMap};

use model::{Map};

pub struct MapRenderer<R: RendererRaw> {
    tileset_texture: Arc<Texture<R>>,
    tileset_first_gid: u32,
    tileset_tiles_amount: Vector2<u32>,
    tileset_uv_per_tile: Vector2<f32>,
}

impl<R: RendererRaw> MapRenderer<R> {
    pub fn new(map: &TMap, map_path: &PathBuf, renderer: &mut Renderer<R>) -> Result<Self, Error> {
        // Load in the map and validate that we can render using it
        if map.tilesets.len() != 1 {
            panic!("Only one tileset per map is supported");
        }
        let tileset = &map.tilesets[0];
        if tileset.images.len() != 1 {
            panic!("Only one impage per tileset is supported");
        }
        let image = &tileset.images[0];

        // We need a path relative to the folder the map is in
        let mut full_image_source = map_path.clone();
        full_image_source.pop();
        full_image_source.push(&image.source);

        // Finally, load in the texture
        let texture = Texture::new()
            .from_file(full_image_source)
            .with_nearest_sampling()
            .build(renderer)?;

        // Calculate all the data we need about this texture to render tiles from it
        let tileset_first_gid = tileset.first_gid;
        let tileset_tiles_amount = Vector2::new(
            image.width as u32 / tileset.tile_width,
            image.height as u32 / tileset.tile_height,
        );
        let tileset_uv_per_tile = Vector2::new(
            1.0 / tileset_tiles_amount.x as f32,
            1.0 / tileset_tiles_amount.y as f32
        );

        Ok(MapRenderer {
            tileset_texture: texture,
            tileset_first_gid,
            tileset_tiles_amount,
            tileset_uv_per_tile,
        })
    }

    pub fn render(
        &self, map: &Map, batches: &mut Vec<RenderBatch<R>>, camera_size: Vector2<f32>
    ) {
        let mut batch = RenderBatch::new(ShaderMode::Texture(self.tileset_texture.clone()));

        let last_pos = Vector2::new(
            (camera_size.x / 32.0).ceil(),
            (camera_size.y / 32.0).ceil(),
        ).cast();

        for layer in map.layers() {
            // TODO: Limit tile rendering to only the visible tiles, not just a fixed amount
            for y in 0..last_pos.y {
                for x in 0..last_pos.x {
                    let position = Point2::new(x, y).cast();
                    let mut tile = layer.tile(position.cast());

                    // We need to adjust the tile's id for looking up in the tileset, but a 0 means
                    // there's nothing there.
                    if tile == 0 { continue };
                    tile -= self.tileset_first_gid;

                    let source_position: Point2<f32> = Point2::new(
                        tile % self.tileset_tiles_amount.x,
                        tile / self.tileset_tiles_amount.x,
                    ).cast();

                    // Add the tile to be rendered
                    batch.push_rectangle(DrawRectangle {
                        destination: Rectangle::new(
                            position * 32.0,
                            (position + Vector2::new(1.0, 1.0)) * 32.0
                        ),
                        texture_source: Some(Rectangle::new(
                            Point2::new(
                                source_position.x * self.tileset_uv_per_tile.x,
                                source_position.y * self.tileset_uv_per_tile.y,
                            ),
                            Point2::new(
                                (source_position.x + 1.0) * self.tileset_uv_per_tile.x,
                                (source_position.y + 1.0) * self.tileset_uv_per_tile.y,
                            ),
                        )),
                        .. DrawRectangle::default()
                    });
                }
            }
        }

        batches.push(batch);
    }
}
