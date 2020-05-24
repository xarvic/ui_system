#version 330

uniform vec2 display;

in vec2 position;
in vec2 coord;
out vec2 v_coord;

void main() {
    gl_Position = vec4(position.x / display.x * 2 - 1, -position.y / display.y * 2 + 1, 0.0, 1.0);
    v_coord = coord;
}