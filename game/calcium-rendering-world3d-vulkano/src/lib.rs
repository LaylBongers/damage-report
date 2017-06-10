extern crate cgmath;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate vulkano;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;
extern crate calcium_rendering_vulkano_shaders;
extern crate calcium_rendering_world3d;

mod geometry_buffer;
mod geometry_renderer;
mod lighting_renderer;
mod mesh;
mod renderer;

pub use self::mesh::{VulkanoMeshBackend, VkVertex};
pub use self::renderer::{VulkanoRendererBackend, BackendMeshes};
