use std::sync::{Arc};

use cgmath::{Vector2, Vector4, BaseNum};

use calcium_rendering::{BackendTypes};

// TODO: #[derive(Debug)]
pub struct RenderBatch<T: BackendTypes> {
    pub mode: BatchMode<T>,
    pub rectangles: Vec<DrawRectangle>,
}

impl<T: BackendTypes> Default for RenderBatch<T> {
    fn default() -> Self {
        RenderBatch {
            mode: BatchMode::Color,
            rectangles: Vec::new(),
        }
    }
}

impl<T: BackendTypes> RenderBatch<T> {
    pub fn empty(&self) -> bool {
        self.rectangles.len() == 0
    }
}

pub enum BatchMode<T: BackendTypes> {
    Color,
    Texture(Arc<T::Texture>),
    Mask(Arc<T::Texture>),
}

#[derive(Debug)]
pub struct DrawRectangle {
    pub destination: Rectangle<i32>,
    // TODO: Support different representations, including pixels
    pub texture_source: Option<Rectangle<f32>>,
    pub color: Vector4<f32>,
}

impl Default for DrawRectangle {
    fn default() -> Self {
        DrawRectangle {
            destination: Rectangle::new(Vector2::new(0, 0), Vector2::new(0, 0)),
            texture_source: None,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle<S: BaseNum> {
    pub start: Vector2<S>,
    pub end: Vector2<S>,
}

impl<S: BaseNum> Rectangle<S> {
    pub fn new(start: Vector2<S>, end: Vector2<S>) -> Self {
        Rectangle {
            start,
            end,
        }
    }

    pub fn start_size(start: Vector2<S>, size: Vector2<S>) -> Self {
        Self::new(start, start + size)
    }
}
