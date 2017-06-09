extern crate cgmath;
extern crate image;
#[macro_use]
extern crate slog;
#[macro_use]
extern crate vulkano;
extern crate wavefront_obj;
extern crate calcium_rendering;
extern crate calcium_rendering_vulkano;
extern crate calcium_rendering_vulkano_shaders;

pub mod vulkano_backend;
mod backend;
mod camera;
mod geometry_buffer;
mod geometry_renderer;
mod lighting_renderer;
mod material;
mod mesh;
mod model;
mod renderer;
mod world;

pub use self::backend::{RendererBackend};
pub use self::camera::{Camera};
pub use self::material::{Material};
pub use self::mesh::{VkVertex, Vertex, Mesh};
pub use self::model::{Model};
pub use self::renderer::{Renderer};
pub use self::world::{World, Entity, Light, EntityId, LightId};