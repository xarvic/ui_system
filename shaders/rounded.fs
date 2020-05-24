#version 330

in vec2 v_roundIndex;
in vec4 v_color;
in float v_pixels;

out vec4 f_color;

void main() {
    f_color = v_color;
    if(v_roundIndex.x > 0 && v_roundIndex.y > 0){
        float round = v_roundIndex.x * v_roundIndex.x + v_roundIndex.y * v_roundIndex.y;
        if(round > (v_pixels + 1) * (v_pixels + 1)){
            discard;
        } else if(round > v_pixels * v_pixels){
            f_color.a *= v_pixels - sqrt(round) + 1;
        }
    }
}