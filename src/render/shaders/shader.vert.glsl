#version 450

// albedo
layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_tex_coord;
// uniforms: camera data
layout(set=0, binding=0) uniform Uniforms {
  mat4 view_proj;
  vec3 cam_pos;
  vec3 cam_dir;
};

layout(location=0) out vec3 v_position;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec2 v_tex_coord;
layout(location=3) out vec3 v_cam_pos;
layout(location=4) out vec3 v_cam_dir;

void main() {
  gl_Position = view_proj * vec4(a_position, 1.0);
  v_position = a_position;
  v_normal = a_normal;
  v_tex_coord = a_tex_coord;
  v_cam_pos = cam_pos;
  v_cam_dir = cam_dir;
}
