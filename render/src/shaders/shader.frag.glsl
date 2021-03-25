#version 450

layout(set=0, binding=0) uniform Uniforms {
  // camera data
  mat4 view_proj;
  vec3 cam_pos;
  vec3 cam_dir;
  // ambient light
  vec3 al_color;
  float al_intensity;
};
// texture data
layout(set=1, binding=0) uniform texture2D t_diffuse;
layout(set=1, binding=1) uniform sampler s_diffuse;
// material data
layout(set=2, binding=0) uniform Material {
  vec3 u_ambient;
  vec3 u_diffuse;
  vec3 u_specular;
  float u_shininess;
};
// light data
layout(set=3, binding=0) uniform PointLight {
  vec3 pl_position;
  vec3 pl_color;
  float pl_intensity;
};

layout(location=0) in vec3 v_position;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec2 v_tex_coord;

layout(location=0) out vec4 f_color;

void main() {
  // pixel is behind camera, we hide it
  vec3 view_dir = normalize(cam_pos - v_position);
  if (dot(view_dir, cam_dir) > 0.0) { discard; }
  // uv has some problem, idk why
  vec4 obj_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coord);
  vec3 normal = normalize(v_normal);
  vec3 light_dir = normalize(pl_position - v_position);
  vec3 half_dir = normalize(light_dir + view_dir);
  // simple blinn phong
  vec3 diffuse = u_diffuse * max(dot(light_dir, normal), 0.0);
  vec3 specular = u_specular * pow(max(dot(normal, half_dir), 0.0), u_shininess);
  vec3 ambient = u_ambient * 0.5 + al_color * al_intensity * 0.5;

  vec3 result = (ambient * 0.2 + (diffuse + specular) * 0.8) * pl_color * pl_intensity * obj_color.xyz;
  f_color = vec4(result, obj_color.a);
}
