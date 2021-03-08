use iced::Application;

mod application;
mod core;
mod examples;
mod graphics;
mod resources;

fn main() -> iced::Result {
    let args = core::cmd::Builder::get_args().get_matches();
    let settings = application::Settings::medium(args);

    application::EngineApp::run(settings)
}
