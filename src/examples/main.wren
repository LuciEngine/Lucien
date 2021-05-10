import "graphics" for Graphics, Light

var start = Fn.new {
  System.print("Scripting with wren")
  var position = Graphics.new_vec3(3, 1, 2)
  var color = Graphics.new_vec3(0.5, 0.4, 0.6)
  var light = Graphics.new_light(position, color)
  System.print(" light: %(light.fmt)")
}

var update = Fn.new {}
