#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;
layout (location = 2) in vec3 Normal;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

out VS_OUTPUT {
    vec2 tex_coord;
    vec3 normal;
    vec3 frag_pos;
} OUT;

void main() {
    gl_Position = proj * view * model * vec4(Position, 1.0);
    OUT.tex_coord = TexCoord;
    OUT.normal = mat3(transpose(inverse(view * model))) * Normal;
    OUT.frag_pos = (view * model * vec4(Position, 1.0)).xyz;
}
