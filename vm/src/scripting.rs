use anyhow::{anyhow, Context, Result};
use lucien_core::logger::logger;
use lucien_core::resources::Project;
use ruwren::{BasicFileLoader, FunctionSignature, Handle, ModuleScriptLoader, VMConfig, VMWrapper, ModuleLibrary};
use slog::{error, info};
use std::rc::Rc;
use crate::printer::LogPrinter;

static DEFAULT_SCRIPT: &str = r##"
var start = Fn.new {
    System.print("No start function defined!")
}
var update = Fn.new { }
"##;

#[derive(Debug, Clone)]
pub struct Scripting {
    vm: VMWrapper,
    src: String,
}

// we must create wren modules here because they are private
static GRAPHICS_MODULE_SRC: &str = include_str!("wren/graphics.wren");

use ruwren::create_module;
create_module!(
    class("Vec3") crate::graphics::WrenVec3 => vec3 {
        instance(getter "fmt") fmt
    }

    class("Light") crate::graphics::Light => light {
        instance(getter "position") position,
        instance(getter "color") color,
        instance(getter "fmt") fmt
    }

    class("Graphics") crate::graphics::Graphics => cg {
        static(fn "new_vec3", 3) new_vec3,
        static(fn "new_light", 4) new_light
    }

    module => graphics
);

// interpret and get update and start function;
// use default empty function if nothing;
// update camera, scene light position;
impl Scripting {
    pub fn new(project: &Project) -> Result<Self> {
        let root = project.path("").unwrap();
        let mut loader = BasicFileLoader::new().base_dir(root);
        let src = loader
            .load_script(String::from("main"))
            .unwrap_or(DEFAULT_SCRIPT.to_string());

        let mut lib = ModuleLibrary::new();
        graphics::publish_module(&mut lib);

        let vm = VMConfig::new()
            .enable_relative_import(true)
            .printer(LogPrinter)
            .script_loader(loader)
            .library(&lib)
            .build();

        vm.interpret("graphics", GRAPHICS_MODULE_SRC).context("Failed to load wren module")?;

        Ok(Self { vm, src })
    }

    pub fn as_ref(&mut self) -> &Self {
        &*self
    }

    // reload script
    // todo hotload
    pub fn init(&self) {
        match self.vm.interpret("main", &self.src) {
            Ok(_) => {}
            Err(e) => {
                error!(logger(), "{}", e);
            }
        };
        info!(logger(), "script loaded");
    }

    // find `start` fn in main.wren
    pub fn start_fn(&self) -> Option<Rc<Handle>> {
        self.vm.execute(|vm| {
            if vm.has_variable("main", "start") {
                vm.ensure_slots(1);
                vm.get_variable("main", "start", 0);
                Some(self.vm.get_slot_handle(0))
            } else {
                error!(logger(), "script doesn't have `start` method");
                None
            }
        })
    }

    // find `update` fn in main.wren
    pub fn update_fn(&self) -> Option<Rc<Handle>> {
        self.vm.execute(|vm| {
            if vm.has_variable("main", "update") {
                vm.ensure_slots(1);
                vm.get_variable("main", "update", 0);
                Some(self.vm.get_slot_handle(0))
            } else {
                error!(logger(), "script doesn't have `update` method");
                None
            }
        })
    }

    pub fn call(&self, handle: Option<Rc<Handle>>) -> Result<()> {
        let fn_call = self
            .vm
            .make_call_handle(FunctionSignature::new_function("call", 0));
        self.vm.set_slot_handle(0, handle.as_ref().unwrap());
        let res = self.vm.call_handle(&fn_call);
        if let Err(e) = res {
            error!(logger(), "* [wren] {}", e);
            return Err(anyhow!("wren runtime error"));
        };
        Ok(())
    }
}
