use std::sync::{Arc};
use std::collections::{HashMap};

use cgmath::{Vector2, Vector4, Point2};
use rusttype::gpu_cache::{Cache};
use rusttype::{Font, Scale};
use image::{GrayImage, GenericImage, ImageBuffer, Luma};
use calcium_rendering::{Renderer, Error};
use calcium_rendering::texture::{Texture};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, SampleMode, Rectangle};

use flowy::style::{Style};
use flowy::{Ui, ElementId, ElementCursorState, Element, ElementText};

pub struct FlowyRenderer<R: Renderer> {
    glyph_cache: Cache,
    glyph_image: GrayImage,
    glyph_texture: Arc<Texture<R>>,
    text_cache: HashMap<ElementId, RenderBatch<R>>,
}

impl<R: Renderer> FlowyRenderer<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        let glyph_cache = Cache::new(512, 512, 0.1, 0.1);
        let glyph_image = GrayImage::from_raw(512, 512, vec![0u8; 512*512]).unwrap();
        let glyph_texture = Texture::new()
            // We will never use this initial texture, so just use something cheap
            .from_greyscale_bytes(&vec![0u8; 8*8], Vector2::new(8, 8))
            .as_single_channel()
            .build(renderer)?;

        Ok(FlowyRenderer {
            glyph_cache,
            glyph_image,
            glyph_texture,
            text_cache: HashMap::new(),
        })
    }

    pub fn draw(
        &mut self, ui: &mut Ui, viewport_size: Vector2<f32>, renderer: &mut R,
    ) -> Result<Vec<RenderBatch<R>>, Error> {
        let mut batcher = Batcher::new();

        // Calculate positioning in the element tree, this needs to be done before rendering so any
        // changes are applied, and so input can use the values for click detection.
        ui.update_layout(viewport_size);

        // Clear unused entries in the text cache
        self.text_cache.retain(|id, _| ui.elements.get(*id).is_some());

        // Draw all the elements recursively starting at the root
        self.draw_element(ui.elements.root_id(), ui, &mut batcher, renderer)?;

        // Make sure all cached batches have the same text texture, this will only matter for the
        //  next frame, but it should clean up some stale textures.
        // TODO: Add something to Texture that just overwrites its data.
        for entry in &mut self.text_cache {
            entry.1.mode = ShaderMode::Mask(self.glyph_texture.clone(), SampleMode::Nearest);
        }

        Ok(batcher.finish())
    }

    fn draw_element(
        &mut self, element_id: ElementId, ui: &mut Ui, batcher: &mut Batcher<R>, renderer: &mut R,
    ) -> Result<(), Error> {
        {
            let element = &mut ui.elements[element_id];

            draw_element_box(element, batcher);
            draw_element_text(
                &ui.fonts,
                element_id, element,
                &mut self.glyph_cache, &mut self.glyph_image, &mut self.glyph_texture,
                batcher, &mut self.text_cache, renderer
            )?;
        }

        // Now go through all the children as well
        for child_id in ui.elements.children_of(element_id).clone() {
            self.draw_element(child_id, ui, batcher, renderer)?;
        }

        Ok(())
    }
}

fn draw_element_box<R: Renderer>(element: &Element, batcher: &mut Batcher<R>) {
    let style = &element.style;

    // If this element is focused, its color should be overwritten with active_color
    let color = if element.focused() {
        style.active_color
            .or(style.hover_color)
            .or(style.background_color)
    } else {
        // Check which color this element is
        match element.cursor_state() {
            ElementCursorState::None => style.background_color,
            ElementCursorState::Hovering => style.hover_color.or(style.background_color),
            ElementCursorState::Held => style.active_color
                .or(style.hover_color)
                .or(style.background_color),
        }
    };

    // Draw a rect for the background if we've got a color
    if let Some(ref color) = color {
        // Draw the rectangle
        batcher.current_batch.rectangle(DrawRectangle {
            destination: element.positioning().container.clone(),
            color: Vector4::new(color.red, color.green, color.blue, color.alpha),
            .. DrawRectangle::default()
        });
    }
}

fn draw_element_text<R: Renderer>(
    fonts: &Vec<Font>,
    id: ElementId, element: &mut Element,
    glyph_cache: &mut Cache, glyph_image: &mut GrayImage, glyph_texture: &mut Arc<Texture<R>>,
    batcher: &mut Batcher<R>, text_cache: &mut HashMap<ElementId, RenderBatch<R>>, renderer: &mut R,
) -> Result<(), Error> {
    // TODO: Glyph positioning should be done during layouting in Ui and cached for future frames,
    //  so text height can be used for automatic layouting as well.

    if element.text.is_some() {
        let font = fonts.get(element.style.text_font.0).expect("Unable to find font on element");
        batcher.next_batch(retrieve_or_create_batch(
            id, element, font,
            glyph_cache, glyph_image, glyph_texture,
            text_cache, renderer,
        )?);

        // Finish off this batch and start on a color batch again
        // TODO: Instead make the batcher know when it should finish off a batch
        batcher.next_batch(RenderBatch::new(ShaderMode::Color));
    }

    Ok(())
}

fn retrieve_or_create_batch<R: Renderer>(
    id: ElementId, element: &mut Element, font: &Font,
    glyph_cache: &mut Cache, glyph_image: &mut GrayImage, glyph_texture: &mut Arc<Texture<R>>,
    text_cache: &mut HashMap<ElementId, RenderBatch<R>>, renderer: &mut R,
) -> Result<RenderBatch<R>, Error> {
    let container = element.positioning().container.clone();
    let text = element.text.as_mut().unwrap();
    let style = &element.style;

    if !text.cache_stale && text.cache_rect == container {
        if let Some(cached_batch) = text_cache.get(&id) {
            return Ok(cached_batch.clone())
        }
    }

    // Couldn't find something in the cache, generate a new batch
    let batch = generate_text_batch(
        text, style, font,
        glyph_cache, glyph_image, glyph_texture,
        renderer,
    )?;

    // Store the batch and mark on the element what its data is
    text_cache.insert(id, batch.clone());
    text.cache_stale = false;
    text.cache_rect = container.clone();
    Ok(batch)
}

fn generate_text_batch<R: Renderer>(
    text: &ElementText, style: &Style, font: &Font,
    glyph_cache: &mut Cache, glyph_image: &mut GrayImage, glyph_texture: &mut Arc<Texture<R>>,
    renderer: &mut R,
) -> Result<RenderBatch<R>, Error> {
    // If the text size is too small, we can't render anything
    if style.text_size <= 0.5 {
        return Ok(RenderBatch::new(ShaderMode::Color))
    }

    // Translate the cached glyphs back into regular glyphs
    let scale = Scale::uniform(style.text_size);
    let mut glyphs = Vec::new();
    for cached_glyph in text.cached_glyphs().unwrap() {
        let glyph = font.glyph(cached_glyph.0).unwrap();
        glyphs.push(glyph.scaled(scale).positioned(cached_glyph.1));
    }

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
        *glyph_texture = Texture::new()
            .from_greyscale_bytes(&glyph_image, Vector2::new(512, 512))
            .as_single_channel()
            .build(renderer)?;
    }

    // Set the texture in the render batch
    let mut batch = RenderBatch::new(ShaderMode::Mask(glyph_texture.clone(), SampleMode::Nearest));

    // Actually render the text
    let c = style.text_color;
    let text_color = Vector4::new(c.red, c.green, c.blue, c.alpha);
    for glyph in glyphs.iter() {
        if let Ok(Some((uv_rect, screen_rect))) = glyph_cache.rect_for(0, glyph) {
            // Push this glyph into this draw batch
            batch.rectangle(DrawRectangle {
                destination: Rectangle {
                    min: Point2::new(screen_rect.min.x as f32, screen_rect.min.y as f32),
                    max: Point2::new(screen_rect.max.x as f32, screen_rect.max.y as f32),
                },
                texture_source: Some(Rectangle {
                    min: Point2::new(uv_rect.min.x, uv_rect.min.y),
                    max: Point2::new(uv_rect.max.x, uv_rect.max.y),
                }),
                color: text_color,
            });
        }
    }

    Ok(batch)
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
