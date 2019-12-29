#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(gl_PointCoord.s, 0.0, 0.5, 1.0);
}