use std::sync::{Arc};

use cgmath::{Vector2};
use image::{ImageBuffer, GrayImage, Luma, GenericImage};
use slog::{Logger};

use conrod::{Color};
use conrod::render::{Text};
use conrod::text::{GlyphCache};
use conrod::text::font::{Id as FontId};

use calcium_rendering::{BackendTypes, Texture};
use calcium_rendering_simple2d::{RenderBatch, BatchMode, DrawRectangle, Rectangle};

use util;

pub struct TextRenderer<T: BackendTypes> {
    glyph_cache: GlyphCache,
    glyph_image: GrayImage,
    glyph_texture: Arc<T::Texture>,
}

impl<T: BackendTypes> TextRenderer<T> {
    pub fn new(log: &Logger, renderer: &mut T::Renderer) -> Self {
        let glyph_cache = GlyphCache::new(1024, 1024, 0.1, 0.1);
        let glyph_image = GrayImage::from_raw(1024, 1024, vec![0u8; 1024*1024]).unwrap();
        let glyph_texture = T::Texture::from_raw_greyscale(
            log, renderer, &vec![0u8; 8*8], Vector2::new(8, 8)
        ); // We will never use this initial texture, so just use something cheap

        TextRenderer {
            glyph_cache,
            glyph_image,
            glyph_texture,
        }
    }

    pub fn push_text(
        &mut self, log: &Logger,
        renderer: &mut T::Renderer, batch: &mut RenderBatch<T>,
        color: Color, text: Text, font_id: FontId,
    ) {
        // Unfortunately this specific text rendering can't be moved into the core simple2d library
        //  because half of it is managed by conrod. Instead we just use the masked solid-color
        //  feature.
        let font_id_u = font_id.index();

        // Get the glyphs we need to render
        // TODO: Support dpi factor
        let positioned_glyphs = text.positioned_glyphs(1.0);

        // Queue up those glyphs in the cache
        for glyph in positioned_glyphs.iter() {
            self.glyph_cache.queue_glyph(font_id.index(), glyph.clone());
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
            // TODO: Remove this weird split_at and find a way to get a slice of all the pixels
            //  without copying
            let (_, data) = glyph_image.split_at(0);
            self.glyph_texture = T::Texture::from_raw_greyscale(
                log, renderer, data, Vector2::new(1024, 1024)
            );
        }

        // Actually set the texture in the render batch
        batch.mode = BatchMode::Mask(self.glyph_texture.clone());

        // Actually render the text
        // TODO: Make use of a glyphs texture
        for glyph in positioned_glyphs.iter() {
            if let Ok(Some((uv_rect, screen_rect))) = self.glyph_cache.rect_for(font_id_u, glyph) {
                // Push this glyph into this draw batch
                batch.rectangle(DrawRectangle {
                    destination: Rectangle {
                        start: Vector2::new(screen_rect.min.x, screen_rect.min.y),
                        end: Vector2::new(screen_rect.max.x, screen_rect.max.y),
                    },
                    texture_source: Some(Rectangle {
                        start: Vector2::new(uv_rect.min.x, uv_rect.min.y),
                        end: Vector2::new(uv_rect.max.x, uv_rect.max.y),
                    }),
                    color: util::color_conrod_to_calcium(color),
                });
            }
        }
    }
}
