#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_position;
layout(location = 1) in vec2 v_uv;

layout(location = 0) out vec2 f_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    f_uv = v_uv;
    gl_Position = vec4(v_position, 0.0, 1.0);
}
