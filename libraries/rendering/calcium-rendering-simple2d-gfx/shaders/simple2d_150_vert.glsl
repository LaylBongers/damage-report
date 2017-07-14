#version 150 core

in vec4 v_pos;
in vec3 v_color;

uniform Transform {
    mat4 u_transform;
};

out vec4 f_color;

void main() {
    f_color = vec4(v_color, 1.0);
    gl_Position = v_pos * u_transform;
}
