use std::sync::{Arc};

use cgmath::{Vector2};
use image::{ImageBuffer, GrayImage, Luma, GenericImage};

use conrod::{Color};
use conrod::render::{Text};
use conrod::text::{GlyphCache};
use conrod::text::font::{Id as FontId};

use calcium_rendering::{Renderer, Texture, Error};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, Rectangle, SampleMode};

use util;

pub struct TextRenderer<R: Renderer> {
    glyph_cache: GlyphCache,
    glyph_image: GrayImage,
    glyph_texture: Arc<Texture<R>>,
}

impl<R: Renderer> TextRenderer<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        let glyph_cache = GlyphCache::new(1024, 1024, 0.1, 0.1);
        let glyph_image = GrayImage::from_raw(1024, 1024, vec![0u8; 1024*1024]).unwrap();
        let glyph_texture = Texture::from_raw_greyscale(
            renderer, &vec![0u8; 8*8], Vector2::new(8, 8)
        )?; // We will never use this initial texture, so just use something cheap

        Ok(TextRenderer {
            glyph_cache,
            glyph_image,
            glyph_texture,
        })
    }

    pub fn push_text(
        &mut self,
        renderer: &mut R, batch: &mut RenderBatch<R>,
        color: Color, text: Text, font_id: FontId,
    ) -> Result<(), Error> {
        // Unfortunately this specific text rendering can't be moved into the core simple2d library
        //  because half of it is managed by conrod. Instead we just use the masked solid-color
        //  feature.
        let font_id_u = font_id.index();

        // Get the glyphs we need to render
        // TODO: Support dpi factor
        let positioned_glyphs = text.positioned_glyphs(1.0);

        // Queue up those glyphs in the cache
        for glyph in positioned_glyphs.iter() {
            self.glyph_cache.queue_glyph(font_id_u, glyph.clone());
        }

        // Now see if we need to create a new glyph cache
        let glyph_image = &mut self.glyph_image;
        let mut changed = false;
        self.glyph_cache.cache_queued(|rect, data| {
            // Create an image from the data we got
            // TODO: See if we can avoid copying all pixel data to create the image
            let new_glyphs_subimage: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(
                rect.width(), rect.height(), data.into()
            ).unwrap();

            // Copy the data into the full glyphs image
            glyph_image.copy_from(&new_glyphs_subimage, rect.min.x, rect.min.y);
            changed = true;
        }).unwrap();

        // If the image has actually changed, update the texture. This is done afterwards because
        //  the cache_queued callback may be called multiple times
        if changed {
            // Upload the glyphs into a texture
            // TODO: Check if we need to convert from sRGB to Linear, calcium takes Linear here
            self.glyph_texture = Texture::from_raw_greyscale(
                renderer, &glyph_image, Vector2::new(1024, 1024)
            )?;
        }

        // Actually set the texture in the render batch
        batch.mode = ShaderMode::Mask(self.glyph_texture.clone(), SampleMode::Nearest);

        // Actually render the text
        for glyph in positioned_glyphs.iter() {
            if let Ok(Some((uv_rect, screen_rect))) = self.glyph_cache.rect_for(font_id_u, glyph) {
                // Push this glyph into this draw batch
                batch.rectangle(DrawRectangle {
                    destination: Rectangle {
                        start: Vector2::new(screen_rect.min.x as f32, screen_rect.min.y as f32),
                        end: Vector2::new(screen_rect.max.x as f32, screen_rect.max.y as f32),
                    },
                    texture_source: Some(Rectangle {
                        start: Vector2::new(uv_rect.min.x, uv_rect.min.y),
                        end: Vector2::new(uv_rect.max.x, uv_rect.max.y),
                    }),
                    color: util::color_conrod_to_calcium(color),
                });
            }
        }

        Ok(())
    }
}
