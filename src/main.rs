use lucien_app as app;
use lucien_core as core;

use anyhow::Result;

fn main() -> Result<()> {
    let cmd = core::cmd::Builder::get_args();
    let args = cmd.get_matches();
    let mut app = app::Application::new(&args)?;
    app.run()
}
