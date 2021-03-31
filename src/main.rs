use lucien_app as app;
use lucien_core as core;
use lucien_render as render;

use anyhow::Result;

fn main() -> Result<()> {
    app::application::Application::run()
}
