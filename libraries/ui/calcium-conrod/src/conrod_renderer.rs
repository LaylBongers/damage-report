use cgmath::{Vector2};

use conrod::{Ui, Color};
use conrod::render::{PrimitiveWalker, PrimitiveKind, Primitives};
use conrod::position::rect::{Rect};

use calcium_rendering::{BackendTypes, WindowRenderer, Renderer};
use calcium_rendering_simple2d::{RenderBatch, DrawRectangle, Rectangle};

use line_renderer::{push_lines};
use text_renderer::{TextRenderer};
use util;

pub struct ConrodRenderer<T: BackendTypes> {
    text_renderer: TextRenderer<T>,
}

impl<T: BackendTypes> ConrodRenderer<T> {
    pub fn new(renderer: &mut T::Renderer) -> Self {
        info!(renderer.log(), "Creating conrod renderer");

        ConrodRenderer {
            text_renderer: TextRenderer::new(renderer),
        }
    }

    pub fn draw_if_changed(
        &mut self,
        renderer: &mut T::Renderer, window: &T::WindowRenderer, ui: &mut Ui,
        batches: &mut Vec<RenderBatch<T>>
    ) {
        if let Some(primitives) = ui.draw_if_changed() {
            *batches = self.draw_primitives(renderer, window, primitives);
        }
    }

    fn draw_primitives(
        &mut self,
        renderer: &mut T::Renderer, window: &T::WindowRenderer, mut primitives: Primitives
    ) -> Vec<RenderBatch<T>> {
        // TODO: Support dpi factor
        let half_size: Vector2<i32> = window.size().cast() / 2;
        let half_size_f: Vector2<f32> = half_size.cast();

        let mut batches = Vec::new();
        let mut batch = Default::default();

        while let Some(prim) = primitives.next_primitive() {
            match prim.kind {
                PrimitiveKind::Rectangle { color } => {
                    self.push_rect(&mut batch, half_size, &prim.rect, color);
                },
                PrimitiveKind::Polygon { color: _, points: _ } => {
                    unimplemented!()
                },
                PrimitiveKind::Lines { color, cap: _, thickness, points } => {
                    push_lines(&mut batch, color, thickness, points, half_size_f);
                },
                PrimitiveKind::Image { image_id: _, color: _, source_rect: _ } => {
                    unimplemented!()
                },
                PrimitiveKind::Text { color, text, font_id } => {
                    // TODO: Re-use the same batch for multiple sequential text draws
                    if !batch.empty() {
                        batches.push(batch);
                        batch = Default::default();
                    }
                    self.text_renderer.push_text(renderer, &mut batch, color, text, font_id);
                    batches.push(batch);
                    batch = Default::default();
                },
                _ => {}
            }
        }

        if !batch.empty() {
            batches.push(batch);
        }

        batches
    }

    fn push_rect(
        &self, batch: &mut RenderBatch<T>, half_size: Vector2<i32>,
        rect: &Rect, color: Color
    ) {
        batch.rectangle(DrawRectangle {
            destination: Rectangle {
                start: Vector2::new(rect.x.start, -rect.y.start).cast() + half_size,
                end: Vector2::new(rect.x.end, -rect.y.end).cast() + half_size,
            },
            texture_source: None,
            color: util::color_conrod_to_calcium(color),
        });
    }
}
