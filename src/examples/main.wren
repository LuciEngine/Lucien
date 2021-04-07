import "graphics" for Graphics, Light

var start = Fn.new {
  System.print("Hello from Wren!")
  var light = Graphics.new_light(3, 1, 2)
  System.print(" color: %(light.color.fmt)")
  System.print(" light: %(light.fmt)")
}

var update = Fn.new {}
