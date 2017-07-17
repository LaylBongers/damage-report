extern crate cgmath;
extern crate calcium_rendering;

mod render_batch;
mod render_target;
mod renderer;
mod types;

pub use render_batch::{RenderBatch, DrawRectangle, Rectangle, ShaderMode, DrawVertex, SampleMode};
pub use render_target::{Simple2DRenderTarget, Simple2DRenderTargetRaw};
pub use renderer::{Simple2DRenderer};
pub use types::{Simple2DTypes};
