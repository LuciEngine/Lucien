#version 450

layout(set=1, binding=0) uniform texture2D t_diffuse;
layout(set=1, binding=1) uniform sampler s_diffuse;

layout(set=2, binding=0) uniform MaterialRaw {
  vec3 u_ambient;
  vec3 u_diffuse;
  vec3 u_specular;
  float u_shininess;
};
layout(set=3, binding=0) uniform Light {
  vec3 l_position;
  vec3 l_color;
};

layout(location=0) in vec3 v_position;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec2 v_tex_coord;
layout(location=3) in vec3 v_cam_pos;
layout(location=4) in vec3 v_cam_dir;

layout(location=0) out vec4 f_color;

void main() {
  vec4 obj_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coord);
  vec3 n = normalize(v_normal);
  vec3 li = normalize(l_position - v_position);
  vec3 v = normalize(v_cam_pos - v_position);
  // pixel is behind camera, we hide it
  if (dot(v, v_cam_dir) >= 0.0) { discard; }
  vec3 h = normalize(li + v);
  vec3 diffuse = u_diffuse * max(dot(li, n), 0.0);
  vec3 specular = u_specular * pow(max(dot(n, h), 0.0), u_shininess);
  vec3 ambient = u_ambient * 0.05;
  vec3 result = (ambient + diffuse + specular) * l_color * obj_color.xyz;
  f_color = vec4(result, obj_color.a);
}
