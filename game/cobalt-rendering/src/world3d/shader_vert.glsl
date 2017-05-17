#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform UniformsData {
    mat4 matrix;
} uniforms;

layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_tex_coords;
layout(location = 2) in vec3 v_normal;

layout(location = 0) out vec2 f_tex_coords;
layout(location = 1) out vec3 f_normal;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    f_tex_coords = v_tex_coords;
    f_normal = v_normal;
    gl_Position = uniforms.matrix * vec4(v_position, 1.0);
}
