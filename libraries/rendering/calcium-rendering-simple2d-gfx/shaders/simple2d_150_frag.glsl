#version 150 core

in vec2 f_uv;
in vec4 f_color;

out vec4 Target0;

void main() {
    Target0 = f_color;
}
