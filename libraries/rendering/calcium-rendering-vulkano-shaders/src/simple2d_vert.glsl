#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform MatrixData {
    mat4 total;
} u_matrix_data;

layout(location = 0) in vec2 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in vec4 v_color;

layout(location = 0) out vec2 f_uv;
layout(location = 1) out vec4 f_color;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    f_uv = v_uv;
    f_color = v_color;
    gl_Position = u_matrix_data.total * vec4(v_position, 0.0, 1.0);
}
