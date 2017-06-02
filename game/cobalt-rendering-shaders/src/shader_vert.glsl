#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform MatrixData {
    mat4 total;
    mat4 model;
} u_matrix_data;

layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_tex_coords;
layout(location = 2) in vec3 v_normal;
layout(location = 3) in vec3 v_tangent;
layout(location = 4) in vec3 v_bitangent;

layout(location = 0) out vec3 f_position;
layout(location = 1) out vec2 f_tex_coords;
layout(location = 2) out vec3 f_normal; //TODO: remove me after normal mapping
layout(location = 3) out mat3 f_tbn;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    vec3 t = normalize(vec3(u_matrix_data.model * vec4(v_tangent, 0.0)));
    vec3 b = normalize(vec3(u_matrix_data.model * vec4(v_bitangent, 0.0)));
    vec3 n = normalize(vec3(u_matrix_data.model * vec4(v_normal, 0.0)));

    f_position = vec3(u_matrix_data.model * vec4(v_position, 1.0));
    f_tex_coords = v_tex_coords;
    f_normal = mat3(transpose(inverse(u_matrix_data.model))) * v_normal;
    f_tbn = mat3(t, b, n);

    gl_Position = u_matrix_data.total * vec4(v_position, 1.0);
}
