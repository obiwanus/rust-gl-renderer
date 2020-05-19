#version 330 core

in VS_OUTPUT {
    vec2 tex_coord;
    vec3 normal;
    vec3 frag_pos;
} IN;

out vec4 Color;

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct MaterialColor {
    vec3 diffuse;
    vec3 specular;
};

struct DirectionalLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float attn_linear;
    float attn_quadratic;
};

uniform Material material;
uniform DirectionalLight directional_light;
#define NUM_POINT_LIGHTS 4
uniform PointLight point_lights[NUM_POINT_LIGHTS];

vec3 calc_directional_light(DirectionalLight light, vec3 normal,
                            vec3 view_direction, MaterialColor mat_color)
{
    vec3 light_direction = normalize(-light.direction);

    // Diffuse
    float diff = max(dot(normal, light_direction), 0.0);

    // Specular
    vec3 reflection = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflection), 0.0), material.shininess);

    // Result
    vec3 ambient = light.ambient * mat_color.diffuse;
    vec3 diffuse = light.diffuse * diff * mat_color.diffuse;
    vec3 specular = light.specular * spec * mat_color.specular;
    return (ambient + diffuse + specular);
}

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos,
                      vec3 view_direction, MaterialColor mat_color)
{
    vec3 light_direction = normalize(light.position - frag_pos);

    // Diffuse
    float diff = max(dot(normal, light_direction), 0.0);

    // Specular
    vec3 reflection = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflection), 1.0), material.shininess);

    // Attenuation
    float light_distance = length(light.position - frag_pos);
    float attenuation = 1.0 / (1.0 + light.attn_linear * light_distance + light.attn_quadratic * (light_distance * light_distance));

    // Result
    vec3 ambient = light.ambient * mat_color.diffuse;
    vec3 diffuse = light.diffuse * diff * mat_color.diffuse;
    vec3 specular = light.specular * spec * mat_color.specular;
    return (ambient + diffuse + specular) * attenuation;
}

void main() {
    vec3 normal = normalize(IN.normal);
    vec3 view_direction = normalize(-IN.frag_pos);

    MaterialColor mat_color;
    mat_color.diffuse = texture(material.diffuse, IN.tex_coord).xyz;
    mat_color.specular = texture(material.specular, IN.tex_coord).xyz;

    vec3 result_color = vec3(0.0);

    // Directional light
    // result_color += calc_directional_light(directional_light, normal, view_direction, mat_color);

    // Point lights
    for (int i = 0; i < NUM_POINT_LIGHTS; i++) {
        result_color += calc_point_light(point_lights[i], normal, IN.frag_pos, view_direction, mat_color);
    }

    Color = vec4(result_color, 1.0);
}
