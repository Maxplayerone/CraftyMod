#version 330
layout (location=0) in vec3 a_Pos;
layout (location=1) in vec2 a_TexCoords;

out vec2 f_TexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    f_TexCoords = a_TexCoords;
    gl_Position = projection * view * model * vec4(a_Pos, 1.0);
}
