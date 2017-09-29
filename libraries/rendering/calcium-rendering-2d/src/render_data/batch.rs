use std::sync::{Arc};

use cgmath::{Vector4, Point2};
use screenmath::{Rectangle};

use calcium_rendering::raw::{RendererRaw};
use calcium_rendering::texture::{Texture};

/// A render batch that can be drawn by a renderer. Represents the equivalent of a single drawcall.
pub struct RenderBatch<R: RendererRaw> {
    /// The shader mode in which a render batch will be drawn.
    pub mode: ShaderMode<R>,

    /// Used to determine what UV coordinates for a full texture are.
    pub uv_mode: UvMode,

    /// The vertices that will be drawn.
    pub vertices: Vec<DrawVertex>,
}

impl<R: RendererRaw> RenderBatch<R> {
    pub fn new(mode: ShaderMode<R>, uv_mode: UvMode) -> Self {
        RenderBatch {
            mode,
            uv_mode,
            vertices: Vec::new(),
        }
    }

    /// Returns true if this render batch has nothing to be drawn.
    pub fn empty(&self) -> bool {
        self.vertices.len() == 0
    }

    /// Adds vertices for a rectangle to this render batch.
    pub fn push_rectangle(&mut self, rect: DrawRectangle) {
        // TODO: make use of uv_mode

        let destination_start_end = rect.destination.min_max().cast();
        let destination_end_start = rect.destination.max_min().cast();
        let uvs = rect.texture_source.unwrap_or(
            Rectangle::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))
        );
        let uvs_min_max = uvs.min_max();
        let uvs_max_min = uvs.max_min();

        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.min.cast(), destination_start_end, destination_end_start],
            if self.uv_mode == UvMode::YDown {
                [uvs.min, uvs_min_max, uvs_max_min]
            } else {
                [uvs_min_max, uvs.min, uvs.max]
            },
            rect.color,
        ));
        self.vertices.extend_from_slice(&DrawVertex::new_triangle(
            [rect.destination.max.cast(), destination_end_start, destination_start_end],
            if self.uv_mode == UvMode::YDown {
                [uvs.max, uvs_max_min, uvs_min_max]
            } else {
                [uvs_max_min, uvs.max, uvs.min]
            },
            rect.color,
        ));
    }
}

impl<R: RendererRaw> Clone for RenderBatch<R> {
    fn clone(&self) -> Self {
        RenderBatch {
            mode: self.mode.clone(),
            uv_mode: self.uv_mode,
            vertices: self.vertices.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UvMode {
    YUp,
    YDown,
}

/// Defines how the renderer should draw vertices.
pub enum ShaderMode<R: RendererRaw> {
    /// Uses only the vertices' colors.
    Color,
    /// Multiplies a texture sampled using the vertices' uvs by the vertices' color.
    Texture(Arc<Texture<R>>),
    /// Uses the vertices' color's RGB and the texture's Alpha.
    Mask(Arc<Texture<R>>),
}

impl<R: RendererRaw> Clone for ShaderMode<R> {
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
    pub texture_source: Option<Rectangle<f32>>,
    /// What solid color this rectangle will be drawn with.
    pub color: Vector4<f32>,
}

impl DrawRectangle {
    /// Creates a new rectangle that will draw the entire texture.
    pub fn full_texture(destination: Rectangle<f32>) -> Self {
        DrawRectangle {
            destination,
            texture_source: None,
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}
