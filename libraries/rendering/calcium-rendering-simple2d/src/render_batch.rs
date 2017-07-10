use std::sync::{Arc};

use cgmath::{Vector2, Vector4};

use calcium_rendering::{BackendTypes};

#[derive(Debug)]
pub struct RenderBatch<T: BackendTypes> {
    pub texture: Option<Arc<T::Texture>>,
    pub rectangles: Vec<DrawRectangle>,
}

impl<T: BackendTypes> Default for RenderBatch<T> {
    fn default() -> Self {
        RenderBatch {
            texture: None,
            rectangles: Vec::new(),
        }
    }
}

impl<T: BackendTypes> RenderBatch<T> {
    pub fn empty(&self) -> bool {
        self.rectangles.len() == 0
    }
}

#[derive(Debug)]
pub struct DrawRectangle {
    pub destination: Rectangle,
    pub color: Vector4<f32>,
}

impl Default for DrawRectangle {
    fn default() -> Self {
        DrawRectangle {
            destination: Rectangle::new(Vector2::new(0, 0), Vector2::new(0, 0)),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub start: Vector2<i32>,
    pub end: Vector2<i32>,
}

impl Rectangle {
    pub fn new(start: Vector2<i32>, end: Vector2<i32>) -> Self {
        Rectangle {
            start,
            end,
        }
    }

    pub fn start_size(start: Vector2<i32>, size: Vector2<i32>) -> Self {
        Self::new(start, start + size)
    }
}
