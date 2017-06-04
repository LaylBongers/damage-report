#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec2 f_uv;
layout(location = 2) in vec3 f_normal;
layout(location = 3) in mat3 f_tbn;

layout(location = 0) out vec4 o_position;
layout(location = 1) out vec4 o_diffuse;

void main() {
    o_position = vec4(f_position, 1.0);
    o_diffuse = vec4(f_uv, 0.0, 1.0);
}
