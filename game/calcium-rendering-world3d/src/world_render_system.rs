pub struct WorldRenderSystem {
    backend: Box<WorldRenderBackend>,
}

impl WorldRenderSystem {
    pub fn new(backend: Box<WorldRenderBackend>) -> WorldRenderSystem {
        WorldRenderSystem {
            backend,
        }
    }
}

pub trait WorldRenderBackend {
}
