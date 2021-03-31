use crate::widgets::UserInterface;
use crate::{GlobalState, Backend, Frontend};
use crate::message::Message;

use iced_winit::winit;
use winit::{
    event::{ Event, WindowEvent },
    event_loop::{ControlFlow, EventLoop},
};

use anyhow::Result;

// run an application
// todo make this a trait so you can customize the application
// todo generic Message type
pub struct Application {
}

impl Application {
    pub fn run() -> Result<()> {
        // create event loop
        let event_loop = EventLoop::<Message>::with_user_event();

        // create winit window
        let mut glob = GlobalState::new(&event_loop);
        let mut backend = Backend::new(&glob);
        let mut frontend = Frontend::new(&glob, UserInterface::new());

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
                        backend.render(&glob, &frame.output, &frontend).expect("3D render failed");
                        // UI render
                        frontend.render(&glob, &frame.output, &backend).expect("UI render failed");
                    }
                }
                Event::WindowEvent { event, .. } => {
                    // handle window events, changes states in glob so UI + backend
                    // could access the changes.
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
                    if let Some(event) = iced_winit::conversion::window_event(
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
