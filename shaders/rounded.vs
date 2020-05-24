#version 330

uniform vec2 display;

in vec2 position;
in vec2 round_index;
in vec4 color;
in float pixels;

out vec2 v_roundIndex;
out vec4 v_color;
out float v_pixels;

void main() {
    gl_Position = vec4(position.x / display.x * 2 - 1, -position.y / display.y * 2 + 1, 0.0, 1.0);
    v_color = color;
    v_roundIndex = vec2(round_index.x + pixels, round_index.y + pixels);
    v_pixels = pixels;
}