mod application;
mod core;
mod resources;
mod widgets;

use lucien_render as render;

use iced_wgpu::{wgpu, Backend, Settings, Viewport};
use iced_wgpu::Renderer as IcedRenderer;
use iced_winit::{conversion, futures, winit, Debug, Size};
use iced_native::program;

use futures::executor::block_on;

use futures::task::SpawnExt;
use winit::{
    dpi::PhysicalPosition,
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::widgets::MainUI;
use crate::application::GPUSupport;


struct WindowState {
    window: winit::window::Window,
    viewport: Viewport,
}

impl WindowState {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        let size = window.inner_size();
        let viewport = Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());

        Self {
            window,
            viewport,
        }
    }

    fn get_size(&self) -> [u32; 2] {
        let size = self.window.inner_size();
        [size.width, size.height]
    }

    // recalculate viewport
    fn viewport(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        let viewport = Viewport::with_physical_size(Size::new(size.width, size.height), self.window.scale_factor());
        self.viewport = viewport;
    }
}

struct Frontend {
    cursor_position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    debug: Debug,
    renderer: IcedRenderer,
}

impl Frontend {
    fn new(device: &mut wgpu::Device) -> Self {
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let modifiers = ModifiersState::default();
        // Initialize staging belt and local pool
        let staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
        let local_pool = futures::executor::LocalPool::new();
        // Initialize iced
        let debug = Debug::new();
        let renderer = init_ui_render(device);

        Self {
            cursor_position,
            modifiers,
            staging_belt,
            local_pool,
            debug,
            renderer,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut app = WindowState::new(&event_loop);

    let mut resized = false;
    let (mut device, queue, surface, mut sc) =
        block_on(application::GPUSupport::init_with_window(&app.window)).unwrap();

    // Initialize 3D render
    let (mut render_settings, mut renderer3d) = init_3d_render(&device, &queue);
    let mut iced = Frontend::new(&mut device);

    renderer3d.state.resize(app.get_size(), &device).unwrap();

    let mut state = program::State::new(
        MainUI::new(),
        app.viewport.logical_size(),
        conversion::cursor_position(iced.cursor_position, app.viewport.scale_factor()),
        &mut iced.renderer,
        &mut iced.debug,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        iced.cursor_position = position;
                    }
                    WindowEvent::ModifiersChanged(new_modifiers) => {
                        iced.modifiers = new_modifiers;
                    }
                    WindowEvent::Resized(new_size) => {
                        app.viewport(&new_size);
                        resized = true;
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                // Map window event to iced event
                if let Some(event) =
                    iced_winit::conversion::window_event(&event, app.window.scale_factor(), iced.modifiers)
                {
                    state.queue_event(event);
                }
            }
            Event::MainEventsCleared => {
                // If there are events pending
                if !state.is_queue_empty() {
                    // We update iced
                    let _ = state.update(
                        app.viewport.logical_size(),
                        conversion::cursor_position(iced.cursor_position, app.viewport.scale_factor()),
                        None,
                        &mut iced.renderer,
                        &mut iced.debug,
                    );
                    // and request a redraw
                    app.window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if resized {
                    sc = GPUSupport::create_swap_chain(&app.window, &device, &surface).unwrap();
                    renderer3d.state.resize(app.get_size(), &device).unwrap();
                    resized = false;
                }
                // use the frame from renderer
                let frame = sc.get_current_frame().expect("Next frame");
                let mut encoder = renderer3d.create_encoder(None, &device);

                let program = state.program();
                {
                    render_settings.clear_color = Some(color(program.background_color()));
                    renderer3d.render_external(&frame.output, &render_settings, &device, &queue).unwrap();
                }

                // And then iced on top
                let mouse_interaction = iced.renderer.backend_mut().draw(
                    &mut device,
                    &mut iced.staging_belt,
                    &mut encoder,
                    &frame.output.view,
                    &app.viewport,
                    state.primitive(),
                    &iced.debug.overlay(),
                );

                // Then we submit the work
                iced.staging_belt.finish();
                queue.submit(Some(encoder.finish()));

                // Update the mouse cursor
                app.window
                    .set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));

                // And recall staging buffers
                iced.local_pool
                    .spawner()
                    .spawn(iced.staging_belt.recall())
                    .expect("Recall staging buffers");

                iced.local_pool.run_until_stalled();
            }
            _ => {}
        }
    });
}

fn init_ui_render(device: &mut wgpu::Device) -> IcedRenderer {
    IcedRenderer::new(Backend::new(device, Settings::default()))
}

fn init_3d_render(device: &wgpu::Device, queue: &wgpu::Queue) -> (render::RenderSettings, render::Renderer) {
    let settings = render::RenderSettings::default();
    let renderer = render::Renderer::new(device, queue, &settings).unwrap();

    (settings, renderer)
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
