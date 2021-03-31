use ruwren;
use ruwren::{BasicFileLoader, VMConfig};

fn main() {
    let script_loader = BasicFileLoader::new().base_dir(".");

    let vm = VMConfig::new()
        .enable_relative_import(true)
        .script_loader(script_loader)
        .build();

    let main_script = r##"
    System.print("No main wren function defined!")
    "##;
    match vm.interpret("main", main_script) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            panic!("Unexpected error!");
        }
    }
}
