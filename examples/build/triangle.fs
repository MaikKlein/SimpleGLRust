#version 150
uniform float value;
out vec4 outColor;
in vec3 vert_color;
void main() {
    outColor = vec4(vert_color.r, vert_color.g, vert_color.b, 1.0);
}