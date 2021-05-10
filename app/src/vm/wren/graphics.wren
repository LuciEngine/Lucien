foreign class Light {
  foreign position
  foreign color
  foreign fmt
}

foreign class PointLight {
  foreign position
  foreign color
}

foreign class Vec3 {
  foreign fmt
}

class Graphics {
  foreign static new_vec3(x, y, z)
  foreign static new_light(position, color)
  /* foreign static new_point_light(position, color) */
}
