extern crate cgmath;
extern crate calcium_rendering;
extern crate screenmath;

mod render_batch;
mod render_target;
mod renderer;

// Re-export screenmath types for convenience
pub use screenmath::{Rectangle};

pub use render_batch::{RenderBatch, DrawRectangle, ShaderMode, DrawVertex, SampleMode};
pub use render_target::{Simple2DRenderTarget, Simple2DRenderTargetRaw};
pub use renderer::{Simple2DRenderer};
