#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coord;
layout(set=0, binding=0)
  uniform Uniforms {
    mat4 u_view_proj;
};

layout(location=0) out vec3 v_position;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec2 v_tex_coord;

void main() {
  gl_Position = u_view_proj * vec4(a_position, 1.0);
  v_position = a_position;
  v_normal = a_normal;
  v_tex_coord = a_tex_coord;
}
