extern crate cgmath;
extern crate calcium_rendering;

mod backend_types;
mod render_batch;
mod renderer;

pub use backend_types::{Simple2DBackendTypes};
pub use render_batch::{RenderBatch, DrawRectangle, Rectangle, ShaderMode, DrawVertex};
pub use renderer::{Simple2DRenderer};
