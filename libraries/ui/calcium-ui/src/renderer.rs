use cgmath::{Vector2, Vector4};
use calcium_rendering::{Renderer};
use calcium_rendering_simple2d::{RenderBatch, ShaderMode, DrawRectangle, Rectangle};

use {Ui, ElementId, Position};

pub struct UiRenderer {
}

impl UiRenderer {
    pub fn new() -> Self {
        UiRenderer {
        }
    }

    pub fn draw<R: Renderer>(&mut self, ui: &Ui) -> Vec<RenderBatch<R>> {
        let mut batcher = Batcher::new();

        self.draw_element(ui.root_id(), ui, &mut batcher);

        batcher.finish()
    }

    fn draw_element<R: Renderer>(
        &self, element_id: ElementId, ui: &Ui, batcher: &mut Batcher<R>
    ) {
        // Add this element itself
        let style = ui[element_id].style();

        let mut position = match &style.position {
            &Position::Flow => Vector2::new(0.0, 0.0),
            &Position::Relative(position) => position,
        };
        position += style.margin.left_top();

        // Draw a rect for the background if we've got a background color
        if let Some(ref color) = style.background_color {
            batcher.current_batch.rectangle(DrawRectangle {
                destination: Rectangle::start_size(position, style.size),
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
