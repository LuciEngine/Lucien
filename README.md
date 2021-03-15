# Lucien

3D render using WebGPU. Will supports scripting with [wren](https://wren.io), with some tools.

## Run

For current stage, it loads a mesh, renders it with a texture.

```bash
# run
cargo run [project_root]
# e.g.
cargo run src/examples/data
```

## Roadmap

✔️ means done. ⚠️ means in progress or pending. Others not started yet.

* ✔️ Render
	* ✔️ renders to memory directly.
	* multiple light sources.
	* multiple meshes.
	* more camera + shaders (post processing + compute shader).
* Scripting
	* expose graphics api + game loop.
* Tools
	* thinking hard on it...

## 🔨 Essential Tools 🔨

* [Iced](https://github.com/hecrj/iced) for window & ui render.
* [Wren](https://github.com/wren-lang/wren) for scripting.

## 📙 References 📙

* [Hazel](https://www.youtube.com/channel/UCQ-W1KE9EYfdxhL6S4twUNw) engine series.
* [GB Studio](https://github.com/chrismaltby/gb-studio).
* [Luxe Engine](http://luxeengine.com/).
* [Game Engine Architecture](https://www.gameenginebook.com).

## Related

Some C++ projects, useful but I didn't quite look into.

* [entt](https://github.com/skypjack/entt)
* [Glen](https://github.com/pulkitjuneja/GlEn)
	* Oh I just looked into it, he's making a similar progress as me...
