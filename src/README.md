## A short story...

Hello, you are here. You must be wondering, what in earth is this project?? What the heck are in those directories? What they do? If I'm interested in a particular thing, where should I look at? ... Okay! Here're some hints!

First, you need to search code. And I have a recommendation: [ag](https://github.com/ggreer/the_silver_searcher). Read the intro, install it, it's very simple to use. If you are missing at any point, just type:

```bash
# This will lead you to the entrance
ag -G ".rs$" "keyword_in_regex"
```

Lucien is a project I use to study crafting game engine, so it's simple. Maybe stupid. The modules provide:

* core: logger, engine event, command line parser, and maybe math
	* but I used an external crate so it's not like other engines
* render: the data structures for rendering, and the actual render logic, shaders are also compiled here
	* ~~for now it has only a cpu raster I copied from another project~~
	* I have done rewriting with wgpu
	* later I'll copy more shader code from other places to make it functional
	* there's a [bug](https://github.com/hecrj/iced/issues/779) in my frontend dependency so I can't integrate my render engine with UI.
* resources: provide a layer to serialze objects and deserialze them to filesystem
* application: defines the engine config, and hook it up to iced app. you could call it entrypoint. there's not much substance here, the exciting things are probably in graphics.
* examples: some dirty in progress widgets and render tests.
