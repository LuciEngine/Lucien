mod application;
mod core;
mod resources;

use lucien_render as render;
use lucien_app as app;

use anyhow::Result;

fn main() -> Result<()> {
    app::application::Application::run()
}
