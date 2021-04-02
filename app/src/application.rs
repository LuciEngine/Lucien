use crate::message::Message;
use crate::widgets::UserInterface;
use crate::{Backend, Frontend, GlobalState};

use anyhow::{anyhow, Context, Result};
use slog::info;
use spin_sleep;
use std::path::PathBuf;
use std::sync::Arc;

use iced_winit::{
    conversion,
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
};

use lucien_core as core;
use lucien_core::resources::{Project, ResourceLoader};
use lucien_core::logger::logger;
use lucien_vm::Scripting;

static VERSION: &str = env!("CARGO_PKG_VERSION");

// run an application
// todo make this a trait so you can customize the application
// todo generic Message type
pub struct Application {
    // project can be shared too?
    project: Option<Project>,
    // wren scripting
    vm: Scripting,
}

impl Application {
    pub fn new(args: &core::ArgFlags) -> Result<Self> {
        let root = args.value_of("project").unwrap();
        let mut proj = Project::new().base_dir(root);
        proj.create_or_load()
            .context("Failed to create or load project")?;
        let vm = Scripting::new(&proj).context("Failed to start vm")?;

        Ok(Self {
            project: Some(proj),
            vm,
        })
    }

    pub fn project(&self) -> Result<&Project> {
        match self.project.as_ref() {
            Some(proj) => Ok(proj),
            _ => Err(anyhow!("failed to get project")),
        }
    }

    pub fn loader(&self) -> Result<Arc<dyn ResourceLoader>> {
        match self.project()?.loader() {
            Some(loader) => Ok(loader),
            _ => Err(anyhow!("failed to get loader")),
        }
    }

    pub fn path(&self, name: &str) -> Option<PathBuf> {
        self.project.as_ref().unwrap().path(name)
    }

    pub fn run(&mut self) -> Result<()> {
        // create event loop
        let event_loop = EventLoop::<Message>::with_user_event();

        // create resource loader
        let loader = self.loader().context("Failed to get resource loader")?;
        // create ui layout
        let ui = UserInterface::new();
        // create winit window
        let mut glob = GlobalState::new(&event_loop);
        let mut backend = Backend::new(&glob, loader).context("Failed to create backend")?;
        let mut frontend = Frontend::new(&glob, ui).context("Failed to create frontend")?;
        info!(logger(), "Window creation successful.");

        glob.window
            .set_title(format!("lucien v{}", VERSION).as_str());

        // wake up main loop on tick and dispatch a custom event
        // from a different thread.
        let proxy = event_loop.create_proxy();
        std::thread::spawn(move || {
            loop {
                // todo use actual frame rate
                spin_sleep::sleep(std::time::Duration::from_millis(10));
                proxy.send_event(Message::Tick).ok();
            }
        });
        // cache engine customized events
        // thank goodness we don't have event loops on
        // different threads so we don't need to use an
        // asynchronous channel for messages
        let mut messages: Vec<Message> = Vec::new();

        // main loop to [produce, handle, and consume]
        // native window events + engine events
        info!(logger(), "Running main loop.");

        self.vm.start();
        self.vm.update();

        event_loop.run(move |event, _, control_flow| {
            // when events are all handled, wait until next event arrives
            // WaitUntil can be useful but I didn't know it was there before
            *control_flow = ControlFlow::Wait;

            // todo synchronize frame rate, drop frame if something takes too long
            match event {
                // *handle* user customized events,
                // cache them in a vector so we *consume* them later;
                // if it is required to be async, then use something like
                // tokio::sync::mpsc as the async channel
                Event::UserEvent(msg) => {
                    messages.push(msg);
                }
                // update
                Event::MainEventsCleared => {
                    // consume user events
                    if !messages.is_empty() {
                        for msg in messages.drain(..) {
                            match msg {
                                // here is where scene update should happen
                                // yes, you only ask the window to redraw on each tick
                                Message::Tick => {
                                    // todo spawn a thread, abort it on timeout
                                    // and request render on finish; also pass in logger
                                    // reuse a tokio runtime to spawn async tasks;
                                    // update should be in a separate thread than render thread
                                    backend.update(&glob).expect("3D update");
                                    glob.window.request_redraw();
                                }
                                // todo other user events
                                _ => {}
                            }
                        }
                    }
                    // consume ui events
                    if !frontend.state.is_queue_empty() {
                        // update UI size and cursor position
                        // drawing doesn't happen immediately until we call `render`
                        frontend.update(&glob).expect("UI update");
                    }
                }
                // render
                Event::RedrawRequested(_) => {
                    // render using glob state
                    // if resize needed, we resize both UI & backend
                    if glob.resized {
                        glob.resize().expect("Resize render target");
                        glob.resized = false;
                    };
                    // draw frame for backend + frontend
                    let frame = &glob.sc.get_current_frame().expect("Get next frame failed");
                    {
                        // 3D render
                        backend
                            .render(&glob, &frame.output, &frontend)
                            .expect("3D render failed");
                        // UI render
                        frontend
                            .render(&glob, &frame.output, &backend)
                            .expect("UI render failed");
                    }
                }
                Event::WindowEvent { event, .. } => {
                    // handle window events, changes states in glob so UI + backend
                    // could access the changes. No actual changes are made, until
                    // they are consumed above.
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            frontend.cursor_position = position;
                        }
                        WindowEvent::ModifiersChanged(new_modifiers) => {
                            frontend.modifiers = new_modifiers;
                        }
                        WindowEvent::Resized(new_size) => {
                            glob.viewport(&new_size);
                            glob.resized = true;
                        }
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    };
                    // Send window event to iced event
                    if let Some(event) = conversion::window_event(
                        &event,
                        glob.window.scale_factor(),
                        frontend.modifiers,
                    ) {
                        frontend.state.queue_event(event);
                    }
                }
                _ => {}
            }
        })
    }
}
