extern crate cgmath;
extern crate slog;
extern crate window;
extern crate winit_window;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;

#[cfg(feature = "world3d")]
extern crate calcium_rendering_world3d;
#[cfg(feature = "world3d")]
extern crate calcium_rendering_world3d_vulkano;

#[cfg(feature = "simple2d")]
extern crate calcium_rendering_simple2d;
#[cfg(feature = "simple2d")]
extern crate calcium_rendering_simple2d_vulkano;

mod initializer;
mod runtime;

pub use initializer::{Initializer};
pub use runtime::{run_runtime, Runtime};

#[allow(dead_code)]
pub enum Backend {
    Vulkano,
    GfxOpenGl,
    GfxDirectX,
}
