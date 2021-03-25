use iced::Application;

mod application;
mod core;
mod resources;
mod widgets;

use lucien_render as render;

fn main() -> iced::Result {
    let args = core::cmd::Builder::get_args().get_matches();
    let settings = application::Settings::medium(args);

    application::EngineApp::run(settings)
}
