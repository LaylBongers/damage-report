extern crate slog;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;
extern crate calcium_rendering_world3d;
extern crate calcium_rendering_world3d_vulkano;

mod runtime;

pub use runtime::{run_runtime, StaticGameRuntime, Initializer};

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
