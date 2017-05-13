#version 330 core

uniform mat4 u_matrix;

in vec3 v_position;

out vec3 f_position;

void main() {
    f_position = v_position;
    gl_Position = u_matrix * vec4(v_position, 1.0);
}
