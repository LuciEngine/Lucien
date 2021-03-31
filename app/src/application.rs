use crate::message::Message;
use crate::widgets::UserInterface;
use crate::{Backend, Frontend, GlobalState};

use anyhow::{Result, Context};
use std::sync::Arc;
use std::path::PathBuf;

use iced_winit::{winit, conversion};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use slog::info;

use lucien_core as core;
use lucien_core::resources::Project;

// run an application
// todo make this a trait so you can customize the application
// todo generic Message type
pub struct Application {
    // logger can be shared by threads
    logger: Arc<core::Logger>,
    project: Option<Project>,
}

impl Application {
    pub fn new(args: &core::ArgFlags) -> Result<Self> {
        let root = args.value_of("project").unwrap();
        let logger = Arc::new(core::logger::CoreLogBuilder::new().get_logger());
        let mut proj = Project::new(Arc::clone(&logger)).base_dir(root);
        proj.create_or_load().context("Failed to create or load project")?;

        Ok(Self {
            logger,
            project: Some(proj),
        })
    }

    pub fn logger(&self) -> &Arc<core::Logger> {
        &self.logger
    }

    pub fn project(&self) -> &Option<Project> {
        &self.project
    }

    pub fn loader(&self) -> &dyn core::resources::ResourceLoader {
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

    pub fn run(&mut self) -> Result<()> {
        // create event loop
        let event_loop = EventLoop::<Message>::with_user_event();

        // create winit window
        let mut glob = GlobalState::new(&event_loop);
        let mut backend = Backend::new(&glob);
        let mut frontend = Frontend::new(&glob, UserInterface::new());
        info!(&self.logger, "Window creation successful.");

        // wake up main loop on tick and dispatch a custom event
        // from a different thread.
        let proxy = event_loop.create_proxy();
        std::thread::spawn(move || {
            loop {
                // todo use actual frame rate
                std::thread::sleep(std::time::Duration::from_millis(10));
                proxy.send_event(Message::Tick).ok();
            }
        });
        // cache engine customized events
        let mut messages: Vec<Message> = Vec::new();

        // main loop to handle native winit events + engine events
        info!(&self.logger, "Running main loop.");

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // todo synchronize frame rate, drop frame if something takes too long
            match event {
                // *handle* user customized events,
                // cache them in a vector so we *consume* them later;
                Event::UserEvent(msg) => {
                    messages.push(msg);
                }
                // update
                Event::MainEventsCleared => {
                    // consume user events
                    if !messages.is_empty() {
                        for msg in messages.drain(..) {
                            match msg {
                                Message::Tick => {
                                    backend.update(&glob).expect("3D update");
                                    glob.window.request_redraw();
                                }
                                // todo other user events
                                _ => {}
                            }
                        }
                    }
                    if !frontend.state.is_queue_empty() {
                        // update UI size and cursor position
                        // drawing doesn't happen immediately until we call `render`
                        frontend.update(&glob).expect("UI update");
                        glob.window.request_redraw();
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
                    let frame = &glob.sc.get_current_frame().expect("Next frame failed");
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
