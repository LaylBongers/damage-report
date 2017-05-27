extern crate vulkano;

#[allow(dead_code)]
pub mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/shader_vert.glsl")} }
#[allow(dead_code)]
pub mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/shader_frag.glsl")} }
