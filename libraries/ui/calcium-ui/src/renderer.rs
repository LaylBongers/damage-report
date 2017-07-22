use std::sync::{Arc};

use cgmath::{Vector2, Vector4};
use rusttype::gpu_cache::{Cache};
use rusttype::{Font, FontCollection, Scale, point, PositionedGlyph};
use image::{GrayImage, GenericImage, ImageBuffer, Luma};
use unicode_normalization::{UnicodeNormalization};
use calcium_rendering::{Renderer, Texture, Error};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, SampleMode, Rectangle};

use style::{CursorBehavior};
use {Ui, ElementId, ElementCursorState, Element, ElementText};

pub struct UiRenderer<R: Renderer> {
    glyph_cache: Cache,
    glyph_image: GrayImage,
    glyph_texture: Arc<Texture<R>>,
    font: Font<'static>,
}

impl<R: Renderer> UiRenderer<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        let glyph_cache = Cache::new(1024, 1024, 0.1, 0.1);
        let glyph_image = GrayImage::from_raw(1024, 1024, vec![0u8; 1024*1024]).unwrap();
        let glyph_texture = Texture::from_raw_greyscale(
            renderer, &vec![0u8; 8*8], Vector2::new(8, 8)
        )?; // We will never use this initial texture, so just use something cheap

        let font = FontCollection::from_bytes(
            ::ttf_noto_sans::REGULAR
        ).into_font().unwrap();

        Ok(UiRenderer {
            glyph_cache,
            glyph_image,
            glyph_texture,
            font,
        })
    }

    pub fn draw(
        &mut self, ui: &mut Ui, viewport_size: Vector2<f32>, renderer: &mut R,
    ) -> Result<Vec<RenderBatch<R>>, Error> {
        let mut batcher = Batcher::new();

        // Calculate positioning in the element tree, this needs to be done before rendering so any
        // changes are applied, and so input can use the values for click detection.
        ui.calculate_positioning(viewport_size);

        // Draw all the elements recursively starting at the root
        self.draw_element(ui.root_id(), ui, &mut batcher, renderer)?;

        Ok(batcher.finish())
    }

    fn draw_element(
        &mut self, element_id: ElementId, ui: &Ui, batcher: &mut Batcher<R>, renderer: &mut R,
    ) -> Result<(), Error> {
        let element = &ui[element_id];

        draw_element_box(element, batcher);
        draw_element_text(
            element, &self.font,
            &mut self.glyph_cache, &mut self.glyph_image, &mut self.glyph_texture,
            batcher, renderer
        )?;

        // Now go through all the children as well
        for child_id in ui.children_of(element_id) {
            self.draw_element(*child_id, ui, batcher, renderer)?;
        }

        Ok(())
    }
}

fn draw_element_box<R: Renderer>(element: &Element, batcher: &mut Batcher<R>) {
    let style = &element.style;

    // Check which color this element is
    let color = match element.cursor_state {
        ElementCursorState::None => style.background_color,
        ElementCursorState::Hovering =>
            match style.cursor_behavior {
                CursorBehavior::Clickable { hover, hold: _hold } =>
                    hover.or(style.background_color),
                _ => style.background_color,
            },
        ElementCursorState::Held =>
            match style.cursor_behavior {
                CursorBehavior::Clickable { hover: _hover, hold } =>
                    hold.or(style.background_color),
                _ => style.background_color,
            },
    };

    // Draw a rect for the background if we've got a color
    if let Some(ref color) = color {
        // Draw the rectangle
        batcher.current_batch.rectangle(DrawRectangle {
            destination: element.positioning.rectangle.clone(),
            color: Vector4::new(color.red, color.green, color.blue, color.alpha),
            .. DrawRectangle::default()
        });
    }
}

fn draw_element_text<R: Renderer>(
    element: &Element, font: &Font,
    glyph_cache: &mut Cache, glyph_image: &mut GrayImage, glyph_texture: &mut Arc<Texture<R>>,
    batcher: &mut Batcher<R>, renderer: &mut R,
) -> Result<(), Error> {
    // TODO: Glyph positioning should be done during layouting in Ui and cached for future frames,
    //  so text height can be used automatically for layouting.

    if let ElementText::SingleLine(ref text) = element.text {
        // If the text size is too small, we can't render anything
        if element.style.text_size <= 0.5 {
            return Ok(())
        }

        let glyphs = layout_text(
            text, element.positioning.rectangle.start, font, element.style.text_size
        );

        // Make sure the glyph cache knows what glyphs we need
        for glyph in &glyphs {
            glyph_cache.queue_glyph(0, glyph.clone());
        }

        // Now see if we need to create a new glyph cache
        let mut changed = false;
        glyph_cache.cache_queued(|rect, data| {
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
            *glyph_texture = Texture::from_raw_greyscale(
                renderer, &glyph_image, Vector2::new(1024, 1024)
            )?;
        }

        // Actually set the texture in the render batch
        batcher.next_batch(RenderBatch::new(
            ShaderMode::Mask(glyph_texture.clone(), SampleMode::Nearest)
        ));

        // Actually render the text
        let c = element.style.text_color;
        let text_color = Vector4::new(c.red, c.green, c.blue, c.alpha);
        for glyph in glyphs.iter() {
            if let Ok(Some((uv_rect, screen_rect))) = glyph_cache.rect_for(0, glyph) {
                // Push this glyph into this draw batch
                batcher.current_batch.rectangle(DrawRectangle {
                    destination: Rectangle {
                        start: Vector2::new(screen_rect.min.x as f32, screen_rect.min.y as f32),
                        end: Vector2::new(screen_rect.max.x as f32, screen_rect.max.y as f32),
                    },
                    texture_source: Some(Rectangle {
                        start: Vector2::new(uv_rect.min.x, uv_rect.min.y),
                        end: Vector2::new(uv_rect.max.x, uv_rect.max.y),
                    }),
                    color: text_color,
                });
            }
        }

        // Finish off this batch and start on a color batch again
        // TODO: Instead make the batcher know when it should finish off a batch
        batcher.next_batch(RenderBatch::new(ShaderMode::Color));
    }

    Ok(())
}

fn layout_text<'a>(
    text: &str, position: Vector2<f32>, font: &'a Font, text_size: f32
) -> Vec<PositionedGlyph<'a>> {
    let mut positioned_glyphs = Vec::new();

    let scale = Scale::uniform(text_size);
    let v_metrics = font.v_metrics(scale);

    let mut caret = point(position.x, position.y + v_metrics.ascent);
    let mut last_glyph_id = None;

    // Convert the text to positioned glyphs
    // Normalizing to "Normalized Form C", reduces mojibake
    for c in text.nfc() {
        // Skip control characters in single-line drawing
        if c.is_control() {
            continue;
        }

        // Look up the glyph for this character
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };

        // Add the kerning needed for the last glyph next to this glyph
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());

        // Position the glyph for this character
        let glyph = base_glyph.scaled(scale).positioned(caret);
        /*if let Some(bb) = glyph.pixel_bounding_box() { TODO: Multi-line support
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }*/
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        positioned_glyphs.push(glyph);
    }

    positioned_glyphs
}

struct Batcher<R: Renderer> {
    current_batch: RenderBatch<R>,
    batches: Vec<RenderBatch<R>>,
}

impl<R: Renderer> Batcher<R> {
    fn new() -> Self {
        Batcher {
            current_batch: RenderBatch::new(ShaderMode::Color),
            batches: Vec::new(),
        }
    }

    fn next_batch(&mut self, mut batch: RenderBatch<R>) {
        ::std::mem::swap(&mut batch, &mut self.current_batch);
        if !batch.empty() {
            self.batches.push(batch);
        }
    }

    fn finish(mut self) -> Vec<RenderBatch<R>> {
        if !self.current_batch.empty() {
            self.batches.push(self.current_batch);
        }

        self.batches
    }
}
