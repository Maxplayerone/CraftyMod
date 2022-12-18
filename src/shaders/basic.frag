#version 330
out vec4 FragColor;

in vec2 f_TexCoords;

uniform sampler2D tex0;
uniform sampler2D tex1;

void main() {
    FragColor = mix(texture(tex0, f_TexCoords), texture(tex1, f_TexCoords), 0.2);
}