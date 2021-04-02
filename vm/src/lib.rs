pub mod printer;
use printer::LogPrinter;

use anyhow::Result;
use lucien_core::logger::logger;
use lucien_core::resources::Project;
use ruwren::{BasicFileLoader, FunctionSignature, ModuleScriptLoader, VMConfig, VMWrapper};
use slog::{error, info};

static DEFAULT_SCRIPT: &str = r##"
System.print("No main wren function defined!")
"##;

pub struct Scripting {
    vm: VMWrapper,
    src: String,
}

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

        Ok(Self { vm, src })
    }

    // reload script
    pub fn reload(&self) {
        info!(logger(), "reload script");
    }

    // get handle functions from script
    pub fn start(&self) {
        match self.vm.interpret("main", &self.src) {
            Ok(_) => {}
            Err(e) => {
                error!(logger(), "{}", e);
            }
        };
        info!(logger(), "vm started");
    }

    // call update
    pub fn update(&self) {
        self.vm.execute(|vm| {
            vm.ensure_slots(1);
            vm.get_variable("main", "Main", 0);
        });
        let main_class = self.vm.get_slot_handle(0);
        let main_function = self
            .vm
            .make_call_handle(FunctionSignature::new_function("main", 0));

        self.vm.set_slot_handle(0, &main_class);
        let res = self.vm.call_handle(&main_function);
        if let Err(e) = res {
            error!(logger(), "{}", e);
        }
    }
}
// interprete and get update and main function
// use default empty function if nothing
// update camera, scene light position
