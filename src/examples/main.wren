import "graphics" for Graphics, Light

var start = Fn.new {
  System.print("Hello from Wren!")
  var color = Graphics.new_vec3(0.5, 0.4, 0.6)
  var light = Graphics.new_light(3, 1, 2, color)
  System.print(" color: %(light.color.fmt)")
  System.print(" light: %(light.fmt)")
}

var update = Fn.new {}
