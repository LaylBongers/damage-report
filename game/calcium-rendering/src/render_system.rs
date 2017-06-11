pub struct RenderSystem {
    backend: Box<RenderBackend>,
}

impl RenderSystem {
    pub fn new(backend: Box<RenderBackend>) -> Self {
        RenderSystem {
            backend,
        }
    }
}

pub trait RenderBackend {
}
