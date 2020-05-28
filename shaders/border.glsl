#vertex
#version 330

uniform vec2 display;

in vec2 position;
in vec2 control;
in vec4 color;
in float width;
in float radius;
in int mode;

out vec4 v_color;
out float v_width;
out float v_radius;
out vec2 v_control;
out int v_mode;

void main() {
    gl_Position = vec4(position.xy, 0.0, 1.0);
    v_color = color;
    v_width = width;
    v_control = control;
    v_mode = mode;
    v_radius = radius;
}

#geometry
#version 330

#define M_PI 3.1415926535897932384626433832795
#define OFFSET 0.5

uniform vec2 display;

layout(lines) in;
layout(triangle_strip, max_vertices = 12) out;

in vec4 v_color[]; // Output from vertex shader for each vertex
in float v_width[];
in vec2 v_control[];
in float v_radius[];
in int v_mode[];

out vec2 roundIndex;
out vec4 color;
out float vi_border;
out float vo_border;

//transforms a gui coordinate to opengl coordnates
void set_position(vec2 pos) {
    gl_Position = vec4(pos.x * 2 / display.x - 1, -pos.y * 2 / display.y + 1, 0.0, 1.0);
}

void main() {
    color = v_color[0];

    float max_width = max(v_width[0], v_width[1]);

    bool rounded = (v_mode[1] & 0x1) != 0;

    /*if((v_mode[1] & 0x2) != 0 || (v_width[0] == 0 && v_width[1] == 0)){

        EndPrimitive(); //Dosn't work!!!
        TODO: fix EndPrimitive
    }*/

    vec2 start = gl_in[0].gl_Position.xy;
    vec2 end = gl_in[1].gl_Position.xy;

    if((v_mode[1] & 0x2) == 0 && (v_width[0] != 0 || v_width[1] == 0)) {

        if (rounded){

            vo_border = v_radius[1] + OFFSET;
            //start
            color = v_color[0];
            vi_border = max(0, v_radius[1] - v_width[0] - OFFSET);
            roundIndex = vec2(v_radius[1], 0);
            set_position(start);
            EmitVertex();

            //mid (out)
            color = (v_color[0] + v_color[1]) / 2;
            vi_border = max(0, v_radius[1] - (v_width[0] + v_width[1]) / 2 - OFFSET);
            roundIndex = vec2(v_radius[1], v_radius[1]);
            set_position(v_control[1]);
            EmitVertex();

            //mid (in)
            roundIndex = vec2(0, 0);
            set_position(start + end - v_control[1]);
            EmitVertex();

            //end
            color = v_color[1];
            vi_border = max(0, v_radius[1] - v_width[1] - OFFSET);
            roundIndex = vec2(0, v_radius[1]);
            set_position(end);
            EmitVertex();

            //EndPrimitive(); /* Dosn't work!!!*/

        } else {
            vec2 dir = normalize(end - start);
            vec2 norm = vec2(-dir.y, dir.x);
            max_width = max(1.5, max_width);
            float border = max_width + 2;
            vec2 dist = norm * border;
            //All
            vi_border = 1.5;

            //Start
            color = v_color[0];
            vo_border = v_width[0] + 2.5;

            roundIndex = vec2(0, 0);
            set_position(start - norm * 2);
            EmitVertex();

            roundIndex = vec2(border + 2, 0);
            set_position(start + dist);
            EmitVertex();

            //End
            vo_border = v_width[1] + 2.5;
            color = v_color[1];

            roundIndex = vec2(0, 0);
            set_position(end - norm * 2);
            EmitVertex();

            roundIndex = vec2(border + 2, 0);
            set_position(end + dist);
            EmitVertex();

            //EndPrimitive(); /* Dosn't work!!!*/
        }
    }
}

#fragment
#version 330

in vec2 roundIndex;
in vec4 color;
in float vi_border;
in float vo_border;

out vec4 f_color;

void main() {
    f_color = color;

    float round = (roundIndex.x * roundIndex.x) + (roundIndex.y * roundIndex.y);

    if(round > vo_border * vo_border || round < vi_border * vi_border){
        discard;
    } else if(round > (vo_border - 1)*(vo_border - 1)){
        f_color.a *= vo_border - sqrt(round);
    } else if(round < (vi_border + 1)*(vi_border + 1)){
        f_color.a *=  sqrt(round) - vi_border;
        if(f_color.a < 0.1)
            discard;
    }
    //f_color = vec4(1.0, 0.0, 1.0, 1.0);
}