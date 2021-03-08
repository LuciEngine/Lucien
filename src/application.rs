use std::path::PathBuf;
use std::rc::Rc;

use iced::{scrollable, text_input};

use crate::core::logger;
use crate::core::message;
use crate::resources::{Project, ResourceLoader};

pub struct Settings;

type ArgFlags = clap::ArgMatches<'static>;

#[allow(dead_code)]
impl Settings {
    pub fn default(args: ArgFlags) -> iced::Settings<ArgFlags> {
        Settings::large(args)
    }

    pub fn small(args: ArgFlags) -> iced::Settings<ArgFlags> {
        let mut settings = iced::Settings::default();
        settings.window.size = (512, 360);
        settings.flags = args;
        settings
    }

    pub fn medium(args: ArgFlags) -> iced::Settings<ArgFlags> {
        let mut settings = iced::Settings::default();
        settings.window.size = (1024, 720);
        settings.flags = args;
        settings
    }

    pub fn large(args: ArgFlags) -> iced::Settings<ArgFlags> {
        let mut settings = iced::Settings::default();
        settings.window.size = (2048, 1440);
        settings.flags = args;
        settings
    }
}

// Track current widgets
#[derive(Debug, Default)]
struct State {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
}

#[allow(dead_code)]
pub struct EngineApp {
    pub logger: Rc<slog::Logger>,
    project: Option<Project>,
    state: State,
}

// Set runtime context here, including:
// the project it is using, etc.
impl EngineApp {
    pub fn loader(&self) -> &dyn ResourceLoader {
        self.project
            .as_ref()
            .unwrap()
            .loader
            .as_ref()
            .unwrap()
            .as_ref()
    }

    pub fn path(&self, name: &str) -> Option<PathBuf> {
        self.project.as_ref().unwrap().path(name)
    }

    fn with_args(args: ArgFlags) -> Self {
        let root = args.value_of("project").unwrap();
        let _logger = Rc::new(logger::CoreLogBuilder::new().get_logger());
        let mut _proj = Project::new(Rc::clone(&_logger)).base_dir(root);
        _proj.create_or_load();

        EngineApp {
            logger: _logger,
            project: Some(_proj),
            state: State::default(),
        }
    }
}

impl iced::Application for EngineApp {
    // thread pool runs commands and subscriptions.
    type Executor = iced::executor::Default;
    // events used by the engine.
    type Message = message::Message;
    // command line flags.
    type Flags = clap::ArgMatches<'static>;

    fn new(args: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (EngineApp::with_args(args), iced::Command::none())
    }

    fn title(&self) -> String {
        "Lucien v0.1.0".into()
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
        match msg {
            _ => {}
        };
        iced::Command::none()
    }

    // refresh window on message
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        // crate::examples::iced_mesh::container(&mut self.state.scroll)
        let bunny = crate::examples::raster::bunny(500, self);
        crate::examples::raster::container(&mut self.state.scroll, bunny)
    }
}
