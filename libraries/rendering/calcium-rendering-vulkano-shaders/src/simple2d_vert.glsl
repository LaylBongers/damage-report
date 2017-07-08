#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform MatrixData {
    mat4 total;
} u_matrix_data;

layout(location = 0) in vec2 v_position;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = u_matrix_data.total * vec4(v_position, 0.0, 1.0);
}
