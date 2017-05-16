#version 330 core

uniform mat4 u_matrix;

in vec3 v_position;
in vec2 v_tex_coords;
in vec3 v_normal;

out vec2 f_tex_coords;
out vec3 f_normal;

void main() {
    f_tex_coords = v_tex_coords;
    f_normal = v_normal;
    gl_Position = u_matrix * vec4(v_position, 1.0);
}
