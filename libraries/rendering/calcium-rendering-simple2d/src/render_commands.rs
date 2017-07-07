use cgmath::{Vector2};

#[derive(Default)]
pub struct RenderCommands {
    pub rectangles: Vec<(Vector2<i32>, Vector2<i32>)>
}

impl RenderCommands {
    pub fn rectangle(&mut self, start: Vector2<i32>, size: Vector2<i32>) {
        self.rectangles.push((start, size));
    }
}
