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
use conrod::{Ui};
use conrod::render::{PrimitiveWalker, PrimitiveKind};
use palette::{Rgba};
use palette::pixel::{Srgb};
use slog::{Logger};

pub struct ConrodRenderer {
}

impl ConrodRenderer {
    pub fn new(log: &Logger) -> Self {
        info!(log, "Creating conrod renderer");

        ConrodRenderer {
        }
    }

    pub fn draw_ui<T: BackendTypes>(
        &self, window: &T::WindowRenderer, ui: &mut Ui
    ) -> Vec<RenderBatch> {
        let half_size: Vector2<i32> = window.size().cast() / 2;
        let mut batch = RenderBatch::default();

        let mut prims = ui.draw();
        while let Some(prim) = prims.next_primitive() {
            match prim.kind {
                PrimitiveKind::Rectangle { color } => {
                    let r = prim.rect;
                    batch.rectangles.push(Rectangle {
                        start: Vector2::new(r.x.start, r.y.start).cast() + half_size,
                        size: Vector2::new(r.x.end - r.x.start, r.y.end - r.y.start).cast(),
                        color: color_conrod_to_calcium(color),
                    });
                },
                PrimitiveKind::Text { color, text: _text, font_id: _font_id } => {
                    // TODO: Actually render text
                    let r = prim.rect;
                    batch.rectangles.push(Rectangle {
                        start: Vector2::new(r.x.start, r.y.start).cast() + half_size,
                        size: Vector2::new(r.x.end - r.x.start, r.y.end - r.y.start).cast(),
                        color: color_conrod_to_calcium(color),
                    });
                },
                _ => {}
            }
        }

        vec!(batch)
    }
}

fn color_conrod_to_calcium(color: ::conrod::Color) -> Vector4<f32> {
    let c = color.to_rgb();
    let c = Srgb::with_alpha(c.0, c.1, c.2, c.3);
    let c: Rgba = c.into();
    Vector4::new(c.red, c.green, c.blue, c.alpha)
}
