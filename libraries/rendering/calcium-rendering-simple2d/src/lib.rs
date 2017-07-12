extern crate cgmath;
extern crate calcium_rendering;

mod render_batch;
mod renderer;
mod types;

pub use render_batch::{RenderBatch, DrawRectangle, Rectangle, ShaderMode, DrawVertex};
pub use renderer::{Simple2DRenderer};
pub use types::{Simple2DTypes};
