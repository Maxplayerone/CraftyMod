#version 330
layout (location=0) in vec2 a_Pos;
layout(location=1) in vec3 a_Color;

out vec3 f_Color;

void main() {
    f_Color = a_Color;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}