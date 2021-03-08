# Lucien

This is the engine I create as my course project. It should render 3D graphics, supports scripting with [wren](https://wren.io), and has a flexible ui.

## 🚥 Build 🚥

For current stage, I render a bunny.

```bash
# build
cargo build
# run
cargo run [project_root]
# e.g.
cargo run src/examples/data
```

## 🐢 Milestones 🐢

✔️ means done. ⚠️ means in progress or pending. Others not started yet.

* Render [in-progress]
	* ✔️ logger
	* layers
	* events
		* Use iced native events.
	* ✔️ resources
	* ⚠️ 2D
		* Maybe hand over it to iced?
	* ⚠️ 3D
		* Done a basic raster shader.
		* Need to compile to GPU.
		* Ray trace is tempting.
* Scripting
	* Game loop
* Tools
	* Thinking on it...

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
