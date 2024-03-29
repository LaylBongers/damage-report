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
    pub fn push_rectangle(
        &mut self,
        destination: Rectangle<f32>, texture_source: Rectangle<f32>, color: Vector4<f32>,
    ) {
        let destination_start_end = destination.min_max().cast();
        let destination_end_start = destination.max_min().cast();

        let uvs = texture_source;
        let uvs_min_max = uvs.min_max();
        let uvs_max_min = uvs.max_min();
        let (tri1_uvs, tri2_uvs) = (
            [uvs.min, uvs_min_max, uvs_max_min],
            [uvs.max, uvs_max_min, uvs_min_max],
        );

        // Add the two triangles for this quad
        self.vertices.push(DrawVertex::new(destination.min.cast(), tri1_uvs[0], color));
        self.vertices.push(DrawVertex::new(destination_start_end, tri1_uvs[1], color));
        self.vertices.push(DrawVertex::new(destination_end_start, tri1_uvs[2], color));

        self.vertices.push(DrawVertex::new(destination.max.cast(), tri2_uvs[0], color));
        self.vertices.push(DrawVertex::new(destination_end_start, tri2_uvs[1], color));
        self.vertices.push(DrawVertex::new(destination_start_end, tri2_uvs[2], color));
    }

    /// Adds vertices for a rectangle to this render batch, assuming the entire texture should be
    /// drawn.
    pub fn push_rectangle_full_texture(
        &mut self,
        destination: Rectangle<f32>
    ) {
        let texture_source = if self.uv_mode == UvMode::YDown {
            Rectangle::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))
        } else {
            Rectangle::new(Point2::new(1.0, 1.0), Point2::new(0.0, 0.0))
        };
        self.push_rectangle(
            destination,
            texture_source,
            Vector4::new(1.0, 1.0, 1.0, 1.0),
        );
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
}
