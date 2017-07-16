use std::sync::{Arc};

use cgmath::{Vector2, Vector4, BaseNum};

use calcium_rendering::{Types, Texture};

/// A render batch that can be drawn by a renderer. Represents the equivalent of a single drawcall.
// TODO: #[derive(Debug)]
pub struct RenderBatch<T: Types> {
    /// The shader mode in which a render batch will be drawn.
    pub mode: ShaderMode<T>,
    /// The vertices that will be drawn.
    pub vertices: Vec<DrawVertex>,
}

impl<T: Types> RenderBatch<T> {
    /// Returns true if this render batch has nothing to be drawn.
    pub fn empty(&self) -> bool {
        self.vertices.len() == 0
    }

    /// Adds vertices for a rectangle to this render batch.
    pub fn rectangle(&mut self, rect: DrawRectangle) {
        let destination_start_end = rect.destination.start_end().cast();
        let destination_end_start = rect.destination.end_start().cast();
        let uvs = rect.texture_source.unwrap_or(
            Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0))
        );
        let uvs_start_end = uvs.start_end();
        let uvs_end_start = uvs.end_start();

        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.start.cast(), destination_start_end, destination_end_start],
            [uvs.start, uvs_start_end, uvs_end_start],
            rect.color,
        ));
        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.end.cast(), destination_end_start, destination_start_end],
            [uvs.end, uvs_end_start, uvs_start_end],
            rect.color,
        ));
    }
}

impl<T: Types> RenderBatch<T> {
    pub fn new(mode: ShaderMode<T>) -> Self {
        RenderBatch {
            mode,
            .. RenderBatch::default()
        }
    }
}

impl<T: Types> Default for RenderBatch<T> {
    fn default() -> Self {
        RenderBatch {
            mode: ShaderMode::Color,
            vertices: Vec::new(),
        }
    }
}

/// Defines how the renderer should draw vertices.
pub enum ShaderMode<T: Types> {
    /// Uses only the vertices' colors.
    Color,
    /// Multiplies a texture sampled using the vertices' uvs by the vertices' color.
    Texture(Arc<Texture<T>>, SampleMode),
    /// Uses the vertices' color's RGB and the texture's Alpha.
    Mask(Arc<Texture<T>>, SampleMode),
}

/// TODO: This type should be changed to a Sampler resource that should be exposed and implemented
///  at the level of calcium-renderer.
pub enum SampleMode {
    Linear,
    Nearest,
}

/// A vertex that can be used to draw on screen.
#[derive(Debug, Clone)]
pub struct DrawVertex {
    /// The 2D position of the vertex in screen pixel coordinates, starting at the top left.
    pub position: Vector2<f32>,
    /// The UV values of the vertex, in Texture and Mask batch mode to sample the texture.
    pub uv: Vector2<f32>,
    /// The color of this vertex, used differently in different batch modes. This color is in
    /// linear color space, rather than sRGB.
    pub color: Vector4<f32>,
}

impl DrawVertex {
    /// Creates a new vertex.
    pub fn new(position: Vector2<f32>, uv: Vector2<f32>, color: Vector4<f32>) -> Self {
        DrawVertex {
            position: position,
            uv: uv,
            color: color,
        }
    }

    /// Creates a triangle of new vertices, with one flat color.
    pub fn new_triangle(
        positions: [Vector2<f32>; 3], uvs: [Vector2<f32>; 3], color: Vector4<f32>
    ) -> [Self; 3] {
        [
            DrawVertex::new(positions[0], uvs[0], color),
            DrawVertex::new(positions[1], uvs[1], color),
            DrawVertex::new(positions[2], uvs[2], color),
        ]
    }
}

/// A rectangle that can be drawn on screen.
#[derive(Debug)]
pub struct DrawRectangle {
    /// Where on screen this rectangle will be drawn.
    pub destination: Rectangle<f32>,
    /// Where in a texture this rectangle should sample from.
    // TODO: Support other representations than normalized UVs in some way, such as pixels. This
    //  perhaps should not be implemented on the DrawRectangle.
    pub texture_source: Option<Rectangle<f32>>,
    /// What solid color this rectangle will be drawn with.
    pub color: Vector4<f32>,
}

impl DrawRectangle {
    /// Creates a new rectangle that will draw the entire texture.
    pub fn new(destination: Rectangle<f32>) -> Self {
        DrawRectangle {
            destination,
            .. DrawRectangle::default()
        }
    }
}

impl Default for DrawRectangle {
    fn default() -> Self {
        DrawRectangle {
            destination: Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)),
            texture_source: None,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

/// A rectangle defined by start and end coordinates.
#[derive(Debug)]
pub struct Rectangle<S: BaseNum> {
    pub start: Vector2<S>,
    pub end: Vector2<S>,
}

impl<S: BaseNum> Rectangle<S> {
    /// Creates a new rectangle.
    pub fn new(start: Vector2<S>, end: Vector2<S>) -> Self {
        Rectangle {
            start,
            end,
        }
    }

    /// Creates a new rectangle from a start coordinate and a size.
    pub fn start_size(start: Vector2<S>, size: Vector2<S>) -> Self {
        Self::new(start, start + size)
    }

    /// Returns a new vector with the start's X and the end's Y.
    pub fn start_end(&self) -> Vector2<S> {
        Vector2::new(self.start.x, self.end.y)
    }

    /// Returns a new vector with the end's X and the start's Y.
    pub fn end_start(&self) -> Vector2<S> {
        Vector2::new(self.end.x, self.start.y)
    }
}
