extern crate calcium_rendering;
extern crate calcium_rendering_simple2d;
extern crate cgmath;
extern crate conrod;
extern crate palette;
#[macro_use]
extern crate slog;

use calcium_rendering::{BackendTypes, WindowRenderer};
use calcium_rendering_simple2d::{RenderBatch, Rectangle};
use cgmath::{Vector2, Vector4};
use conrod::{Ui, Color};
use conrod::render::{PrimitiveWalker, PrimitiveKind, Text};
use conrod::text::{GlyphCache};
use conrod::text::font::{Id as FontId};
use conrod::position::rect::{Rect};
use palette::{Rgba};
use palette::pixel::{Srgb};
use slog::{Logger};

pub struct ConrodRenderer {
    glyph_cache: GlyphCache,
}

impl ConrodRenderer {
    pub fn new(log: &Logger) -> Self {
        info!(log, "Creating conrod renderer");

        let glyph_cache = GlyphCache::new(1024, 1024, 0.1, 0.1);

        ConrodRenderer {
            glyph_cache
        }
    }

    pub fn draw_ui<T: BackendTypes>(
        &mut self, window: &T::WindowRenderer, ui: &mut Ui
    ) -> Vec<RenderBatch> {
        // TODO: Support dpi factor
        let half_size: Vector2<i32> = window.size().cast() / 2;
        let mut batch = RenderBatch::default();

        let mut prims = ui.draw();
        while let Some(prim) = prims.next_primitive() {
            match prim.kind {
                PrimitiveKind::Rectangle { color } => {
                    self.push_rect(&mut batch, half_size, &prim.rect, color);
                },
                PrimitiveKind::Text { color, text, font_id } => {
                    self.push_text(&mut batch, color, text, font_id);
                },
                _ => {}
            }
        }

        vec!(batch)
    }

    fn push_rect(
        &self, batch: &mut RenderBatch, half_size: Vector2<i32>,
        rect: &Rect, color: Color
    ) {
        batch.rectangles.push(Rectangle {
            start: Vector2::new(rect.x.start, rect.y.start).cast() + half_size,
            size: Vector2::new(rect.x.end - rect.x.start, rect.y.end - rect.y.start).cast(),
            color: color_conrod_to_calcium(color),
        });
    }

    fn push_text(
        &mut self, batch: &mut RenderBatch,
        color: Color, text: Text, font_id: FontId,
    ) {
        let font_id_u = font_id.index();

        // Get the glyphs we need to render
        // TODO: Support dpi factor
        let positioned_glyphs = text.positioned_glyphs(1.0);

        // Queue up those glyphs in the cache
        for glyph in positioned_glyphs.iter() {
            self.glyph_cache.queue_glyph(font_id.index(), glyph.clone());
        }

        // Now see if we need to create a new glyph cache
        self.glyph_cache.cache_queued(|_rect, _data| {
            // TODO: Actually load the data into a texture
        }).unwrap();

        // Actually render the text
        for glyph in positioned_glyphs.iter() {
            if let Ok(Some((_uv_rect, screen_rect))) = self.glyph_cache.rect_for(font_id_u, glyph) {
                // Push this glyph into this draw batch
                batch.rectangles.push(Rectangle {
                    start: Vector2::new(screen_rect.min.x, screen_rect.min.y),
                    size: Vector2::new(screen_rect.width(), screen_rect.height()),
                    color: color_conrod_to_calcium(color),
                });
            }
        }
    }
}

fn color_conrod_to_calcium(color: ::conrod::Color) -> Vector4<f32> {
    let c = color.to_rgb();
    let c = Srgb::with_alpha(c.0, c.1, c.2, c.3);
    let c: Rgba = c.into();
    Vector4::new(c.red, c.green, c.blue, c.alpha)
}
