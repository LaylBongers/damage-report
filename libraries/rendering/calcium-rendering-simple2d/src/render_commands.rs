use cgmath::{Vector2, Vector4};

#[derive(Default, Debug)]
pub struct RenderCommands {
    pub rectangles: Vec<Rectangle>
}

#[derive(Debug)]
pub struct Rectangle {
    pub start: Vector2<i32>,
    pub size: Vector2<i32>,
    pub color: Vector4<f32>,
}
