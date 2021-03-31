use std::path::PathBuf;
use std::sync::Arc;

use iced::scrollable;

// use crate::core::logger;
// use crate::core::message;
use crate::render::{RenderSettings, Renderer};
use crate::resources::{Project, ResourceLoader};

// use futures::executor::block_on;

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

use iced_native::program::Program;
use iced_winit::{ Command, Subscription, Mode, Color, };
pub trait Application : Program {
    type Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>);

    fn title(&self) -> String;

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn mode(&self) -> Mode {
        Mode::Windowed
    }

    fn background_color(&self) -> Color {
        Color::WHITE
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

// run an application
// takes integrated state
pub fn _run() {
    use iced_winit::winit;
    use iced_futures::Runtime;
    use iced_winit::Proxy;

    use winit::{
        event_loop::{ControlFlow, EventLoop},
    };
    use crate::ui::widgets::MainInterface;
    use crate::ui::{IntegrateState, Backend, Frontend};

    // create event loop
    //   https://docs.rs/winit/0.24.0/winit/event_loop/struct.EventLoopProxy.html
    let event_loop = EventLoop::new();
    let mut glob = IntegrateState::new(&event_loop);
    let mut backend = Backend::new(&glob);
    let mut frontend = Frontend::new(&glob, MainInterface::new());

    // use proxy to send custom events
    let proxy = event_loop.create_proxy();
    let mut runtime = {
        let proxy = Proxy::new(event_loop.create_proxy());
        let executor = iced_futures::executor::Tokio::new().unwrap();

        Runtime::new(executor, proxy)
    };
    // use runtime to track executor & subscription `iced_futures::Runtime`
    //   https://docs.rs/iced_futures/0.2.0/iced_futures/struct.Runtime.html
    // create winit window
    // create event sender, send event in event_loop

    // create instance closure (surface, sc, receiver, etc.)
    // use it to actually handle the event loop
}

async fn _run_instance() {

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

    // async fn render_to_buffer(
    //     renderer: Arc<Renderer>, settings: Arc<RenderSettings>,
    // ) -> Result<RgbaBuffer, Error> {
    //     renderer.render(&settings, &device, &queue).context("Failed to render.")?;
    //     renderer
    //         .read_to_buffer()
    //         .context("Failed to write to render buffer.")?;
    //     // @todo this buffer convert is slow
    //     let buffer = renderer
    //         .as_rgba()
    //         .await
    //         .context("Failed to convert to rgba.")?;
    //     Ok(buffer)
    // }
    //
    // async fn render_to_file(buffer: Arc<RgbaBuffer>) -> Result<()> {
    //     buffer.save("window.png")?;
    //     Ok(())
    // }
}

// impl iced::Application for EngineApp {
//     // thread pool runs commands and subscriptions.
//     type Executor = iced::executor::Default;
//     // events used by the engine.
//     type Message = message::Message;
//     // command line flags.
//     type Flags = clap::ArgMatches<'static>;
//
//     fn new(args: Self::Flags) -> (Self, iced::Command<Self::Message>) {
//         (EngineApp::with_args(args).unwrap(), iced::Command::none())
//     }
//
//     fn title(&self) -> String {
//         format!("Lucien v{:?}", env!("CARGO_PKG_VERSION")).into()
//     }
//
//     fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
//         match msg {
//             Self::Message::Tick => {
//                 if self.state.ticks >= 100 {
//                     self.state.ticks = 0;
//                 }
//                 // update game logic
//                 // in a separate thread, not in the main thread
//                 // iced::Command::perform(
//                 //     EngineApp::render_update(self.renderer.clone()),
//                 //     Self::Message::UpdateComplete,
//                 // );
//
//                 // if it's busy, don't commit anything, this ensures the render
//                 // operation is only executed one at a time; otherwise, the thread
//                 // will panic.
//                 if self.state.busy_render {
//                     iced::Command::none()
//                 } else {
//                     self.state.busy_render = true;
//                     self.state.ticks += 1;
//
//                     iced::Command::perform(
//                         EngineApp::render_to_buffer(
//                             self.renderer.clone(),
//                             self.render_settings.clone(),
//                         ),
//                         Self::Message::RenderComplete,
//                     )
//                 }
//             }
//             // once finished render, save them as file
//             Self::Message::RenderComplete(Ok(_buffer)) => {
//                 self.state.busy_render = false;
//                 iced::Command::none()
//                 // iced::Command::perform(
//                 //     EngineApp::render_to_file(Arc::new(buffer)),
//                 //     Self::Message::RenderSaveComplete,
//                 // )
//             }
//             // once finished save, the frame is done, we record the frame rate
//             Self::Message::RenderSaveComplete(Ok(())) => {
//                 // self.state.ticks += 1;
//                 iced::Command::none()
//             }
//             _ => iced::Command::none(),
//         }
//     }
//
//     // ~100 fps
//     fn subscription(&self) -> Subscription<Self::Message> {
//         // todo set self.state idle or update
//         iced_futures::time::every(std::time::Duration::from_millis(10)).map(|_| Self::Message::Tick)
//     }
//
//     // refresh window on message
//     fn view(&mut self) -> iced::Element<Self::Message> {
//         crate::widgets::main_window(&self.state)
//     }
// }
