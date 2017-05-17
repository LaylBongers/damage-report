extern crate vulkano_shaders;

fn main() {
    // building the shaders used in the examples
    vulkano_shaders::build_glsl_shaders([
        ("src/world3d/shader_vert.glsl", vulkano_shaders::ShaderType::Vertex),
        ("src/world3d/shader_frag.glsl", vulkano_shaders::ShaderType::Fragment),
    ].iter().cloned());
}
