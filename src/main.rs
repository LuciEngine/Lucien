mod application;
mod core;
mod resources;
mod widgets;

use lucien_render as render;

use iced_native::program;
use iced_wgpu::Renderer as IcedRenderer;
use iced_wgpu::{wgpu, Viewport};
use iced_winit::{conversion, futures, winit, Debug, Size};

use futures::executor::block_on;
use futures::task::SpawnExt;
use winit::{
    dpi::PhysicalPosition,
    event::{Event, ModifiersState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::application::GPUSupport;
use crate::widgets::MainUI;

struct IntegrateState {
    window: winit::window::Window,
    viewport: Viewport,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    sc: wgpu::SwapChain,
    resized: bool,
}

impl IntegrateState {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        let size = window.inner_size();
        let viewport =
            Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());
        let (device, queue, surface, sc) =
            block_on(application::GPUSupport::init_with_window(&window)).unwrap();

        Self {
            window,
            viewport,
            device,
            queue,
            surface,
            sc,
            resized: false,
        }
    }

    fn get_size(&self) -> [u32; 2] {
        let size = self.window.inner_size();
        [size.width, size.height]
    }

    // recalculate viewport
    fn viewport(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        let viewport = Viewport::with_physical_size(
            Size::new(size.width, size.height),
            self.window.scale_factor(),
        );
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
    state: program::State<MainUI>,
}

impl Frontend {
    fn new(glob: &IntegrateState) -> Self {
        use iced_wgpu::{Backend, Settings};

        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let modifiers = ModifiersState::default();
        // Initialize staging belt and local pool
        let staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
        let local_pool = futures::executor::LocalPool::new();
        // Initialize iced
        let mut debug = Debug::new();
        let mut renderer = IcedRenderer::new(Backend::new(&glob.device, Settings::default()));
        // UI state
        let state = program::State::new(
            MainUI::new(),
            glob.viewport.logical_size(),
            conversion::cursor_position(cursor_position, glob.viewport.scale_factor()),
            &mut renderer,
            &mut debug,
        );

        Self {
            cursor_position,
            modifiers,
            staging_belt,
            local_pool,
            debug,
            renderer,
            state,
        }
    }

    fn update(&mut self, glob: &IntegrateState) {
        self.state.update(
            glob.viewport.logical_size(),
            conversion::cursor_position(self.cursor_position, glob.viewport.scale_factor()),
            None,
            &mut self.renderer,
            &mut self.debug,
        );
    }

    fn render(&mut self, glob: &IntegrateState, target: &wgpu::SwapChainTexture, mut encoder: wgpu::CommandEncoder) {
        let mouse_interaction = self.renderer.backend_mut().draw(
            &glob.device,
            &mut self.staging_belt,
            &mut encoder,
            &target.view,
            &glob.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        );
        // Then we submit the work
        self.staging_belt.finish();
        glob.queue.submit(Some(encoder.finish()));
        // And recall staging buffers
        self.local_pool
            .spawner()
            .spawn(self.staging_belt.recall())
            .expect("Recall staging buffers");
        self.local_pool.run_until_stalled();
        // Update the mouse cursor
        glob.window
            .set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));
    }
}

struct Backend {
    settings: render::RenderSettings,
    renderer: render::Renderer,
}

impl Backend {
    fn new(glob: &IntegrateState) -> Self {
        let settings = render::RenderSettings::new(glob.get_size());
        let renderer = render::Renderer::new(&glob.device, &glob.queue, &settings).unwrap();

        Self {
            settings,
            renderer,
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut glob = IntegrateState::new(&event_loop);
    let mut backend = Backend::new(&glob);
    let mut frontend = Frontend::new(&glob);

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
                    frontend.update(&glob);
                    // and request a redraw
                    glob.window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                if glob.resized {
                    glob.sc = GPUSupport::create_swap_chain(&glob.window, &glob.device, &glob.surface).expect("Resize swap chain");
                    backend.renderer
                        .state
                        .resize(glob.get_size(), &glob.device).expect("Resize backend");
                    glob.resized = false;
                }
                // draw frame for backend + frontend
                let frame = &glob.sc.get_current_frame().expect("Next frame");
                {
                    let program = frontend.state.program();
                    {
                        backend.settings.clear_color = Some(color(program.background_color()));
                        backend.renderer
                            .render_external(&frame.output, &backend.settings, &glob.device, &glob.queue)
                            .expect("Backend 3D render");
                    }
                    // And then iced on top
                    let encoder = backend.renderer.create_encoder(Some("Frontend Encoder"), &glob.device);
                    frontend.render(&glob, &frame.output, encoder);
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
