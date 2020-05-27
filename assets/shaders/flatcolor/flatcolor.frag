#version 330 core

in VS_OUTPUT {
  vec3 normal;
  vec3 frag_pos;
  vec3 color;
}
IN;

out vec4 Color;

struct Material {
  vec3 diffuse;
  vec3 specular;
  float shininess;
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
// uniform PointLight point_light;
uniform DirectionalLight directional_light;

vec3 calc_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction)
{
    vec3 light_direction = normalize(-light.direction);
    vec3 diffuse_color = IN.color;

    // Diffuse
    float diff = max(dot(normal, light_direction), 0.0);

    // Specular
    vec3 reflection = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflection), 0.0), material.shininess);

    // Result
    vec3 ambient = light.ambient * diffuse_color;
    vec3 diffuse = light.diffuse * diff * diffuse_color;
    vec3 specular = light.specular * spec * material.specular;
    return (ambient + diffuse + specular);
}

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_direction) {
  vec3 light_direction = normalize(light.position - frag_pos);

  vec3 diffuse_color = IN.color;
  // vec3 diffuse_color = material.diffuse;

  // Diffuse
  float diff = max(dot(normal, light_direction), 0.0);

  // Specular
  vec3 reflection = reflect(-light_direction, normal);
  float spec = pow(max(dot(view_direction, reflection), 1.0), material.shininess);

  // Attenuation
  float light_distance = length(light.position - frag_pos);
  float attenuation = 1.0 / (1.0 + light.attn_linear * light_distance +
                             light.attn_quadratic * (light_distance * light_distance));

  // Result
  vec3 ambient = light.ambient * diffuse_color;
  vec3 diffuse = light.diffuse * diff * diffuse_color;
  vec3 specular = light.specular * spec * material.specular;
  return (ambient + diffuse + specular) * attenuation;
}

void main() {
  vec3 normal = normalize(IN.normal);
  vec3 view_direction = normalize(-IN.frag_pos);

  vec3 result_color = vec3(0.0);

  // Directional light
  result_color += calc_directional_light(directional_light, normal, view_direction);

//   // Point light
//   result_color += calc_point_light(point_light, normal, IN.frag_pos, view_direction);

  Color = vec4(result_color, 1.0);
}
