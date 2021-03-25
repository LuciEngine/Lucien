## Render

3D render engine, can write to buffer or draw in a winit window. For example, see [here](../examples).

The idea is simple:

* A `Renderer` takes a `RenderSettings`, and changes the data in `RenderState`.
* A `RenderState` contains only data, so a renderer can interpret it.
* The `gpu_data` converts other types to gpu preferred format, and the rest are the data structures used in rendering.

Wgpu is kind of more complicated. You should read their [doc](https://sotrh.github.io/learn-wgpu/) to get an idea, but the basic is it uses `layout` to define the memory structure in gpu, and takes the `view` for specific data structures. `desc` for each data defines how gpu uses them.

## Bugs

* UV is probably wrong.
