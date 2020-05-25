#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
// layout (location = 2) in vec4 Color;

uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;

out VS_OUTPUT {
    vec3 normal;
    vec3 frag_pos;
    vec3 color;
} OUT;

void main() {
    gl_Position = proj * view * model * vec4(Position, 1.0);
    OUT.normal = mat3(transpose(inverse(view * model))) * Normal;
    OUT.frag_pos = (view * model * vec4(Position, 1.0)).xyz;
    // OUT.color = Color.xyz;
    OUT.color = vec3(1.0, 1.0, 1.0);
}
