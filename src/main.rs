mod application;
mod core;
mod resources;

use lucien_render as render;
use lucien_ui as ui;

use iced_wgpu::wgpu;
use iced_winit::winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use ui::widgets::MainInterface;

// todo: make application in ui and expose Application only,
// impl title, subscrption and events inside
// when do I update backend? how do I update it using external script?

fn main() {
    let event_loop = EventLoop::new();
    let mut glob = ui::IntegrateState::new(&event_loop);
    let mut backend = ui::Backend::new(&glob);
    let mut frontend = ui::Frontend::new(&glob, MainInterface::new());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
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
                }

                // Map window event to iced event
                if let Some(event) = iced_winit::conversion::window_event(
                    &event,
                    glob.window.scale_factor(),
                    frontend.modifiers,
                ) {
                    frontend.state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !frontend.state.is_queue_empty() {
                    // We update iced
                    frontend.update(&glob).expect("Update UI");
                    // todo it's jaggy because it's not called at fixed rate
                    backend.update(&glob).expect("Update Scene");
                    // and request a redraw
                    glob.window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if glob.resized {
                    glob.sc = ui::create_swap_chain(&glob.window, &glob.device, &glob.surface)
                        .expect("Resize swap chain");
                    backend
                        .renderer
                        .state
                        .resize(glob.get_size(), &glob.device)
                        .expect("Resize backend");
                    glob.resized = false;
                }
                // draw frame for backend + frontend
                let frame = &glob.sc.get_current_frame().expect("Next frame");
                {
                    let program = frontend.state.program();
                    {
                        backend.settings.clear_color = Some(color(program.background_color()));
                        backend
                            .renderer
                            .render_external(
                                &frame.output,
                                &backend.settings,
                                &glob.device,
                                &glob.queue,
                            )
                            .expect("Backend 3D render");
                    }
                    // And then iced on top
                    let encoder = backend
                        .renderer
                        .create_encoder(Some("Frontend Encoder"), &glob.device);
                    frontend.render(&glob, &frame.output, encoder).expect("Render UI");
                }
            }
            _ => {}
        }
    });
}

fn color(background_color: iced_winit::Color) -> wgpu::Color {
    let [r, g, b, a] = background_color.into_linear();

    wgpu::Color {
        r: r as f64,
        g: g as f64,
        b: b as f64,
        a: a as f64,
    }
}
