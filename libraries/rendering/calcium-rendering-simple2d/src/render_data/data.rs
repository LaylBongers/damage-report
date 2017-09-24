use calcium_rendering::{Renderer};

use render_data::{RenderBatch, Projection};

pub struct RenderData<R: Renderer> {
    pub render_sets: Vec<RenderSet<R>>,
}

impl<R: Renderer> RenderData<R> {
    pub fn new() -> Self {
        RenderData {
            render_sets: Vec::new(),
        }
    }
}

pub struct RenderSet<R: Renderer> {
    pub projection: Projection,
    pub batches: Vec<RenderBatch<R>>,
}

impl<R: Renderer> RenderSet<R> {
    pub fn new(projection: Projection, batches: Vec<RenderBatch<R>>) -> Self {
        RenderSet {
            projection,
            batches,
        }
    }
}
