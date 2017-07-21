use cgmath::{Vector2, Vector4};
use calcium_rendering::{Renderer};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, Rectangle};

use style::{Position, Size};
use {Ui, ElementId};

pub struct UiRenderer {
}

impl UiRenderer {
    pub fn new() -> Self {
        UiRenderer {
        }
    }

    pub fn draw<R: Renderer>(
        &mut self, ui: &mut Ui, viewport_size: Vector2<f32>
    ) -> Vec<RenderBatch<R>> {
        let mut batcher = Batcher::new();

        // Update the root so it has the size of the viewport
        let root_id = ui.root_id();
        ui[root_id].style_mut().size = Size::units(viewport_size.x, viewport_size.y);

        // Draw all the elements recursively starting at the root
        self.draw_element(ui.root_id(), ui, viewport_size, &mut batcher);

        batcher.finish()
    }

    fn draw_element<R: Renderer>(
        &self, element_id: ElementId, ui: &Ui, parent_size: Vector2<f32>, batcher: &mut Batcher<R>,
    ) {
        // Add this element itself
        let style = ui[element_id].style();

        // Calculate the final position of this element
        let mut position = match &style.position {
            &Position::Flow => Vector2::new(0.0, 0.0),
            &Position::Relative(position) => position,
        };
        position += style.margin.left_top();

        // Calculate the final size of this element
        let size = style.size.to_units(parent_size);

        // Draw a rect for the background if we've got a background color
        if let Some(ref color) = style.background_color {

            // Draw the rectangle
            batcher.current_batch.rectangle(DrawRectangle {
                destination: Rectangle::start_size(position, size),
                color: Vector4::new(color.red, color.green, color.blue, color.alpha),
                .. DrawRectangle::default()
            });
        }

        // Now go through all the children as well
        for child_id in ui.children_of(element_id) {
            self.draw_element(*child_id, ui, size, batcher);
        }
    }
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

    fn finish(mut self) -> Vec<RenderBatch<R>> {
        if !self.current_batch.empty() {
            self.batches.push(self.current_batch);
        }

        self.batches
    }
}
