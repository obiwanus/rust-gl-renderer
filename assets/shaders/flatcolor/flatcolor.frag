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

struct PointLight {
  vec3 position;

  vec3 ambient;
  vec3 diffuse;
  vec3 specular;

  float attn_linear;
  float attn_quadratic;
};

uniform Material material;
uniform PointLight point_light;

vec3 calc_point_light(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_direction) {
  vec3 light_direction = normalize(light.position - frag_pos);

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
  vec3 ambient = light.ambient * material.diffuse;
  vec3 diffuse = light.diffuse * diff * material.diffuse;
  vec3 specular = light.specular * spec * material.specular;
  return (ambient + diffuse + specular) * attenuation;
}

void main() {
  vec3 normal = normalize(IN.normal);
  vec3 view_direction = normalize(-IN.frag_pos);

  vec3 result_color = vec3(0.0);

  // Point light
  result_color += calc_point_light(point_light, normal, IN.frag_pos, view_direction);

  Color = vec4(result_color, 1.0);
}

