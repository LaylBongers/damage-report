extern crate cgmath;
extern crate gfx;
extern crate image;
extern crate input;
extern crate slog;
extern crate calcium_rendering;

mod renderer;
mod texture;
mod types;
mod window_renderer;

pub use renderer::{GfxRenderer};
pub use texture::{GfxTexture};
pub use types::{GfxTypes};
pub use window_renderer::{GfxWindowRenderer, GfxFrame};

pub type ColorFormat = ::gfx::format::Rgba8;
pub type DepthFormat = ::gfx::format::DepthStencil;