#version 150 core

uniform Transform {
    mat4 u_transform;
};

in vec2 v_position;
in vec2 v_uv;
in vec4 v_color;

out vec2 f_uv;
out vec4 f_color;

void main() {
    f_uv = v_uv;
    f_color = v_color;
    gl_Position = u_transform * vec4(v_position, 0.0, 1.0);
}
