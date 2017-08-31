use std::sync::{Arc};

use cgmath::{Vector4, Point2};
use screenmath::{Rectangle};

use calcium_rendering::{Renderer};
use calcium_rendering::texture::{Texture};

/// A render batch that can be drawn by a renderer. Represents the equivalent of a single drawcall.
pub struct RenderBatch<R: Renderer> {
    /// The shader mode in which a render batch will be drawn.
    pub mode: ShaderMode<R>,
    /// The vertices that will be drawn.
    pub vertices: Vec<DrawVertex>,
}

impl<R: Renderer> RenderBatch<R> {
    pub fn new(mode: ShaderMode<R>) -> Self {
        RenderBatch {
            mode,
            .. RenderBatch::default()
        }
    }

    /// Returns true if this render batch has nothing to be drawn.
    pub fn empty(&self) -> bool {
        self.vertices.len() == 0
    }

    /// Adds vertices for a rectangle to this render batch.
    pub fn rectangle(&mut self, rect: DrawRectangle) {
        let destination_start_end = rect.destination.min_max().cast();
        let destination_end_start = rect.destination.max_min().cast();
        let uvs = rect.texture_source.unwrap_or(
            Rectangle::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))
        );
        let uvs_start_end = uvs.min_max();
        let uvs_end_start = uvs.max_min();

        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.min.cast(), destination_start_end, destination_end_start],
            [uvs.min, uvs_start_end, uvs_end_start],
            rect.color,
        ));
        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.max.cast(), destination_end_start, destination_start_end],
            [uvs.max, uvs_end_start, uvs_start_end],
            rect.color,
        ));
    }
}

impl<R: Renderer> Default for RenderBatch<R> {
    fn default() -> Self {
        RenderBatch {
            mode: ShaderMode::Color,
            vertices: Vec::new(),
        }
    }
}

impl<R: Renderer> Clone for RenderBatch<R> {
    fn clone(&self) -> Self {
        RenderBatch {
            mode: self.mode.clone(),
            vertices: self.vertices.clone(),
        }
    }
}

/// Defines how the renderer should draw vertices.
pub enum ShaderMode<R: Renderer> {
    /// Uses only the vertices' colors.
    Color,
    /// Multiplies a texture sampled using the vertices' uvs by the vertices' color.
    Texture(Arc<Texture<R>>),
    /// Uses the vertices' color's RGB and the texture's Alpha.
    Mask(Arc<Texture<R>>),
}

impl<R: Renderer> Clone for ShaderMode<R> {
    fn clone(&self) -> Self {
        match *self {
            ShaderMode::Color => ShaderMode::Color,
            ShaderMode::Texture(ref t) => ShaderMode::Texture(t.clone()),
            ShaderMode::Mask(ref t) => ShaderMode::Mask(t.clone()),
        }
    }
}

/// A vertex that can be used to draw on screen.
#[derive(Debug, Clone)]
pub struct DrawVertex {
    /// The 2D position of the vertex in screen pixel coordinates, starting at the top left.
    pub position: Point2<f32>,
    /// The UV values of the vertex, in Texture and Mask batch mode to sample the texture.
    pub uv: Point2<f32>,
    /// The color of this vertex, used differently in different batch modes. This color is in
    /// linear color space, rather than sRGB.
    pub color: Vector4<f32>,
}

impl DrawVertex {
    /// Creates a new vertex.
    pub fn new(position: Point2<f32>, uv: Point2<f32>, color: Vector4<f32>) -> Self {
        DrawVertex {
            position: position,
            uv: uv,
            color: color,
        }
    }

    /// Creates a triangle of new vertices, with one flat color.
    pub fn new_triangle(
        positions: [Point2<f32>; 3], uvs: [Point2<f32>; 3], color: Vector4<f32>
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
            destination: Rectangle::new(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0)),
            texture_source: None,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
