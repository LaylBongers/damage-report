use cgmath::{Vector2, Zero};

use conrod::{Ui, Color, Point};
use conrod::render::{PrimitiveWalker, PrimitiveKind, Primitives};
use conrod::position::rect::{Rect};
use conrod::widget::primitive::shape::triangles::{Triangle, ColoredPoint};
use conrod::color::{Rgba};

use calcium_rendering::{WindowRenderer, Renderer, Error};
use calcium_rendering_2d::{RenderBatch, DrawRectangle, Rectangle, DrawVertex};

use text_renderer::{TextRenderer};
use util;

pub struct ConrodRenderer<R: RendererRaw> {
    text_renderer: TextRenderer<R>,
}

impl<R: RendererRaw> ConrodRenderer<R> {
    pub fn new(renderer: &mut R) -> Result<Self, Error> {
        info!(renderer.log(), "Creating conrod renderer");

        let text_renderer = TextRenderer::new(renderer)?;

        Ok(ConrodRenderer {
            text_renderer,
        })
    }

    pub fn draw_if_changed(
        &mut self,
        renderer: &mut R, window: &R::WindowRenderer, ui: &mut Ui,
    ) -> Result<Option<Vec<RenderBatch<R>>>, Error> {
        let result = if let Some(primitives) = ui.draw_if_changed() {
            Some(self.draw_primitives(renderer, window, primitives)?)
        } else {
            None
        };

        Ok(result)
    }

    fn draw_primitives(
        &mut self,
        renderer: &mut R, window: &R::WindowRenderer, mut primitives: Primitives
    ) -> Result<Vec<RenderBatch<R>>, Error> {
        // TODO: Support dpi factor
        let half_size: Vector2<f32> = window.size().cast() / 2.0;

        let mut batches = Vec::new();
        let mut batch = Default::default();

        while let Some(prim) = primitives.next_primitive() {
            match prim.kind {
                PrimitiveKind::Rectangle { color } => {
                    self.push_rect(&mut batch, half_size, &prim.rect, color);
                },
                PrimitiveKind::TrianglesSingleColor { color, triangles } => {
                    self.push_triangles_single_color(&mut batch, half_size, color, triangles);
                },
                PrimitiveKind::TrianglesMultiColor { triangles } => {
                    self.push_triangles_multi_color(&mut batch, half_size, triangles);
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
                    self.text_renderer.push_text(renderer, &mut batch, color, text, font_id)?;
                    batches.push(batch);
                    batch = Default::default();
                },
                _ => {}
            }
        }

        if !batch.empty() {
            batches.push(batch);
        }

        Ok(batches)
    }

    fn push_rect(
        &self, batch: &mut RenderBatch<R>, half_size: Vector2<f32>,
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

    fn push_triangles_single_color(
        &self, batch: &mut RenderBatch<R>, half_size: Vector2<f32>,
        color: Rgba, triangles: &[Triangle<Point>]
    ) {
        for triangle in triangles {
            let point = triangle.0[0];
            batch.vertices.push(DrawVertex::new(
                Vector2::new(point[0], point[1]).cast() + half_size,
                Vector2::zero(),
                util::color_conrod_rgba_to_calcium(color),
            ));
        }
    }

    fn push_triangles_multi_color(
        &self, batch: &mut RenderBatch<R>, half_size: Vector2<f32>,
        triangles: &[Triangle<ColoredPoint>]
    ) {
        for triangle in triangles {
            let point = triangle.0[0];
            batch.vertices.push(DrawVertex::new(
                Vector2::new(point.0[0], point.0[1]).cast() + half_size,
                Vector2::zero(),
                util::color_conrod_rgba_to_calcium(point.1),
            ));
        }
    }
}
