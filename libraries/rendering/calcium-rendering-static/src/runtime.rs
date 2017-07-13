use calcium_rendering::{Error};

use {Backend, Initializer};

pub fn run_runtime<R: Runtime>(backend: Backend, runtime: R) -> Result<(), Error> {
    match backend {
        Backend::Vulkano =>
            runtime.run(::vulkano::VulkanoInitializer),
        Backend::GfxOpenGl =>
            runtime.run(::gfx_opengl::GfxOpenGlInitializer),
        Backend::GfxDirectX => unimplemented!(),
    }
}

pub trait Runtime {
    fn run<I: Initializer>(self, init: I) -> Result<(), Error>;
}
