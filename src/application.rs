use std::path::PathBuf;
use std::sync::Arc;

use iced::scrollable;

// use crate::core::logger;
use crate::render::{RenderSettings, Renderer};
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
pub struct State {
    pub scroll: scrollable::State,
    // track if renderer is still copying buffer etc.
    busy_render: bool,
    // track frame rate
    pub ticks: u32,
}

#[allow(dead_code)]
pub struct EngineApp {
    pub logger: Arc<slog::Logger>,
    project: Option<Project>,
    state: State,
    render_settings: Arc<RenderSettings>,
    renderer: Arc<Renderer>,
}

// Set runtime context here, including:
// the project it is using, etc.
#[allow(dead_code)]
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

    // // initialization of everything
    // fn with_args(args: ArgFlags) -> Result<Self> {
    //     let root = args.value_of("project").unwrap();
    //     let logger = Arc::new(logger::CoreLogBuilder::new().get_logger());
    //     // init 3D renderer
    //     let (device, queue) = block_on(GPUSupport::init_headless()).unwrap();
    //     let render_settings = Arc::new(RenderSettings::default());
    //     let renderer = Arc::new(Renderer::new(device, queue, render_settings.size)?);
    //     // init project
    //     let mut proj = Project::new(Arc::clone(&logger)).base_dir(root);
    //     proj.create_or_load();
    //
    //     Ok(EngineApp {
    //         logger,
    //         project: Some(proj),
    //         state: State::default(),
    //         render_settings,
    //         renderer,
    //     })
    // }
}
