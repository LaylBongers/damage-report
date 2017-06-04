#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(set = 0, binding = 0) uniform MatrixData {
    mat4 total;
    mat4 model;
} u_matrix_data;

layout(location = 0) in vec3 v_position;
layout(location = 1) in vec2 v_uv;
layout(location = 2) in vec3 v_normal;
layout(location = 3) in vec3 v_tangent;

layout(location = 0) out vec3 f_position;
layout(location = 1) out vec2 f_uv;
layout(location = 2) out vec3 f_normal;
layout(location = 3) out mat3 f_tbn;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    // Calculate the normal mapping data
    vec3 t = normalize(vec3(u_matrix_data.model * vec4(v_tangent, 0.0)));
    vec3 n = normalize(vec3(u_matrix_data.model * vec4(v_normal, 0.0)));
    // Re-orthogonalize T with respect to N
    t = normalize(t - dot(t, n) * n);
    // Then retrieve perpendicular vector B with the cross product of T and N
    // TODO: I'm not sure why I had to invert the result here, but without it
    //  the normals were moved the wrong direction across the bitangent.
    //  Tutorials aren't doing this so I'm not sure what's wrong.
    vec3 b = -cross(n, t);
    f_tbn = mat3(t, b, n);

    // Create all the values the fragment shader will need
    f_position = vec3(u_matrix_data.model * vec4(v_position, 1.0));
    f_uv = v_uv;
    f_normal = mat3(transpose(inverse(u_matrix_data.model))) * v_normal;

    gl_Position = u_matrix_data.total * vec4(v_position, 1.0);
}
