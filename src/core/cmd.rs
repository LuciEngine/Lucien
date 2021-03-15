use clap::{App, Arg};

pub struct Builder;

#[allow(dead_code)]
impl Builder {
    pub fn get_args() -> App<'static, 'static> {
        App::new("Luci Engine")
            .version("0.0.1")
            .arg(
                Arg::with_name("project")
                    .help("Sets the project root")
                    .required(false)
                    .default_value("."),
            )
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
    }
}
