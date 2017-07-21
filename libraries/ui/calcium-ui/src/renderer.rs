use cgmath::{Vector2, Vector4};
use calcium_rendering::{Renderer};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, Rectangle};

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

        // Calculate positioning in the element tree, this needs to be done before rendering so any
        // changes are applied, and so input can use the values for click detection.
        ui.calculate_positioning(viewport_size);

        // Draw all the elements recursively starting at the root
        self.draw_element(ui.root_id(), ui, &mut batcher);

        batcher.finish()
    }

    fn draw_element<R: Renderer>(
        &self, element_id: ElementId, ui: &Ui, batcher: &mut Batcher<R>,
    ) {
        let element = &ui[element_id];
        let style = &element.style;
        let positioning = &element.positioning;

        // Draw a rect for the background if we've got a background color
        if let Some(ref color) = style.background_color {
            // Draw the rectangle
            batcher.current_batch.rectangle(DrawRectangle {
                destination: Rectangle::start_size(positioning.position, positioning.size),
                color: Vector4::new(color.red, color.green, color.blue, color.alpha),
                .. DrawRectangle::default()
            });
        }

        // Now go through all the children as well
        for child_id in ui.children_of(element_id) {
            self.draw_element(*child_id, ui, batcher);
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
