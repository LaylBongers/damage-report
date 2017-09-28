use calcium_rendering::raw::{RendererRaw};

use render_data::{RenderBatch, Projection};

pub struct RenderData<R: RendererRaw> {
    pub render_sets: Vec<RenderSet<R>>,
}

impl<R: RendererRaw> RenderData<R> {
    pub fn new() -> Self {
        RenderData {
            render_sets: Vec::new(),
        }
    }
}

pub struct RenderSet<R: RendererRaw> {
    pub projection: Projection,
    pub batches: Vec<RenderBatch<R>>,
}

impl<R: RendererRaw> RenderSet<R> {
    pub fn new(projection: Projection, batches: Vec<RenderBatch<R>>) -> Self {
        RenderSet {
            projection,
            batches,
        }
    }
}
