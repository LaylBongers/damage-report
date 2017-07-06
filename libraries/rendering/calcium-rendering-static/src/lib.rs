extern crate cgmath;
extern crate slog;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;
extern crate calcium_rendering_world3d;
extern crate calcium_rendering_world3d_vulkano;
extern crate calcium_window;
extern crate calcium_window_winit;

mod initializer;
mod runtime;

pub use initializer::{Initializer};
pub use runtime::{run_runtime, StaticGameRuntime};

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
