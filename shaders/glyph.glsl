#vertex
#version 330

uniform vec2 display;

in vec2 position;
in vec2 coord;
in vec4 color;

out vec4 v_color;
out vec2 v_coord;

void main() {
    gl_Position = vec4(position.x / display.x * 2 - 1, -position.y / display.y * 2 + 1, 0.0, 1.0);
    v_coord = coord;
    v_color = color;
}

#fragment
#version 330

uniform sampler2D tex;


in vec2 v_coord;
in vec4 v_color;

out vec4 f_color;

void main() {
    vec4 c = texture(tex, v_coord);

    f_color = v_color;
    f_color.a *= (1 - c.r) * 1.3;
}