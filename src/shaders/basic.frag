#version 330
out vec4 FragColor;

in vec2 f_TexCoords;

uniform sampler2D tex0;

void main() {
    FragColor = texture(tex0, f_TexCoords);
}