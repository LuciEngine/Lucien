use lucien_app as app;

use anyhow::Result;

fn main() -> Result<()> {
    app::application::Application::run()
}
