#version 150 core

uniform vec2 u_center;
uniform vec3 u_color;
in vec2 pos;
out vec4 v_Color;

void main() {
    v_Color = vec4(u_color, 1.0);
    gl_Position = vec4(u_center + pos, 0.0, 1.0);
}
