use std::sync::{Arc};

use cgmath::{Vector2, Vector4, BaseNum};

use calcium_rendering::{BackendTypes};

// TODO: #[derive(Debug)]
pub struct RenderBatch<T: BackendTypes> {
    pub mode: BatchMode<T>,
    pub triangles: Vec<[DrawVertex; 3]>,
}

impl<T: BackendTypes> RenderBatch<T> {
    pub fn empty(&self) -> bool {
        self.triangles.len() == 0
    }

    pub fn rectangle(&mut self, rect: DrawRectangle) {
        let destination_start_end = rect.destination.start_end().cast();
        let destination_end_start = rect.destination.end_start().cast();
        let uvs = rect.texture_source.unwrap_or(
            Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0))
        );
        let uvs_start_end = uvs.start_end();
        let uvs_end_start = uvs.end_start();

        self.triangles.push(DrawVertex::new_triangle(
            [rect.destination.start.cast(), destination_start_end, destination_end_start],
            [uvs.start, uvs_start_end, uvs_end_start],
            rect.color,
        ));
        self.triangles.push(DrawVertex::new_triangle(
            [rect.destination.end.cast(), destination_end_start, destination_start_end],
            [uvs.end, uvs_end_start, uvs_start_end],
            rect.color,
        ));
    }
}

impl<T: BackendTypes> Default for RenderBatch<T> {
    fn default() -> Self {
        RenderBatch {
            mode: BatchMode::Color,
            triangles: Vec::new(),
        }
    }
}

pub enum BatchMode<T: BackendTypes> {
    Color,
    Texture(Arc<T::Texture>),
    Mask(Arc<T::Texture>),
}

pub struct DrawVertex {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
    pub color: Vector4<f32>,
}

impl DrawVertex {
    pub fn new_triangle(
        positions: [Vector2<f32>; 3], uvs: [Vector2<f32>; 3], color: Vector4<f32>
    ) -> [DrawVertex; 3] {
        [DrawVertex {
            position: positions[0],
            uv: uvs[0],
            color: color,
        }, DrawVertex {
            position: positions[1],
            uv: uvs[1],
            color: color,
        }, DrawVertex {
            position: positions[2],
            uv: uvs[2],
            color: color,
        }]
    }
}

#[derive(Debug)]
pub struct DrawRectangle {
    pub destination: Rectangle<i32>,
    // TODO: Support other representations than normalized UVs, such as pixels
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

    pub fn start_end(&self) -> Vector2<S> {
        Vector2::new(self.start.x, self.end.y)
    }

    pub fn end_start(&self) -> Vector2<S> {
        Vector2::new(self.end.x, self.start.y)
    }
}
