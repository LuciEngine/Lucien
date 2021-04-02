pub mod printer;
use printer::LogPrinter;

use anyhow::Result;
use lucien_core::logger::logger;
use lucien_core::resources::Project;
use ruwren::{BasicFileLoader, FunctionSignature, ModuleScriptLoader, VMConfig, VMWrapper};
use slog::{error, info};

static DEFAULT_SCRIPT: &str = r##"
class Main {
    static start() {
        System.print("No main function defined!")
    }
    static update() { }
}
"##;

pub struct Scripting {
    vm: VMWrapper,
    src: String,
    has_start: bool,
    has_update: bool,
}

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

        let vm = VMConfig::new()
            .enable_relative_import(true)
            .printer(LogPrinter)
            .script_loader(loader)
            .build();

        Ok(Self { vm, src, has_start: false, has_update: false })
    }

    // reload script
    // todo hotload
    pub fn init(&mut self) {
        match self.vm.interpret("main", &self.src) {
            Ok(_) => {}
            Err(e) => {
                error!(logger(), "{}", e);
            }
        };
        info!(logger(), "script loaded");

        // find `start` fn in main.wren
        self.has_start = self.vm.execute(|vm| {
            if vm.has_variable("main", "start") {
                vm.ensure_slots(1);
                vm.get_variable("main", "start", 0);
                true
            }
            else {
                error!(logger(), "script doesn't have `start` method");
                false
            }
        });
        // find `update` fn in main.wren
        self.has_update = self.vm.execute(|vm| {
            if vm.has_variable("main", "update") {
                vm.ensure_slots(1);
                vm.get_variable("main", "update", 0);
                true
            }
            else {
                error!(logger(), "script doesn't have `update` method");
                false
            }
        });
    }

    // call start
    pub fn start(&self) {
        if !self.has_start { return; }
        // get start function from main module and call
        // let main_class = self.vm.get_slot_handle(0);
        let handle = self
            .vm
            .make_call_handle(FunctionSignature::new_function("start", 0));

        // self.vm.set_slot_handle(0, &main_class);
        let res = self.vm.call_handle(&handle);
        if let Err(e) = res {
            error!(logger(), "* [wren] {}", e);
        }
    }

    // call update
    pub fn update(&self) {
        if !self.has_update { return; }
        // let main_class = self.vm.get_slot_handle(0);
        let handle = self
            .vm
            .make_call_handle(FunctionSignature::new_function("update", 1));

        // self.vm.set_slot_handle(0, &main_class);
        let res = self.vm.call_handle(&handle);
        if let Err(e) = res {
            error!(logger(), "* [wren] {}", e);
        }
    }
}
