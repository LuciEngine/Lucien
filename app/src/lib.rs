use lucien_render as render;
use anyhow::{Result, Context};

use iced_native::program;
use iced_wgpu::Renderer as IcedRenderer;
use iced_wgpu::{wgpu, Viewport};
use iced_winit::{conversion, futures, winit, Debug, Size};

use futures::executor::block_on;
use futures::task::SpawnExt;
use winit::{
    dpi::PhysicalPosition,
    event::ModifiersState,
    event_loop::EventLoop,
};

pub mod widgets;
pub mod message;
pub mod application;
use widgets::UserInterface;
use message::Message;

#[allow(dead_code)]
async fn init_headless() -> Result<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        })
        .await
        .context("Failed to request adapter")?;
    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .context("Failed to request device")?;

    Ok((device, queue))
}

async fn init_with_window(
    window: &winit::window::Window,
) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface, wgpu::SwapChain)> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(window) };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        })
        .await
        .context("Request adapter")?;

    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .context("Failed to request device")?;

    let swap_chain =
        create_swap_chain(&window, &device, &surface).context("Failed to create swap_chain")?;

    Ok((device, queue, surface, swap_chain))
}

// Resize swap chain texture size
fn create_swap_chain(
    window: &winit::window::Window, device: &wgpu::Device, surface: &wgpu::Surface,
) -> Result<wgpu::SwapChain> {
    let swap_chain = {
        let size = window.inner_size();

        device.create_swap_chain(
            surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        )
    };
    Ok(swap_chain)
}

struct GlobalState {
    pub window: winit::window::Window,
    pub viewport: Viewport,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub sc: wgpu::SwapChain,
    pub resized: bool,
}

impl GlobalState {
    pub fn new(event_loop: &EventLoop<Message>) -> Self {
        let window = winit::window::Window::new(event_loop).unwrap();
        let size = window.inner_size();
        let viewport =
            Viewport::with_physical_size(Size::new(size.width, size.height), window.scale_factor());
        let (device, queue, surface, sc) = block_on(init_with_window(&window)).unwrap();

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

    pub fn get_size(&self) -> [u32; 2] {
        let size = self.window.inner_size();
        [size.width, size.height]
    }

    // recalculate viewport
    pub fn viewport(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        let viewport = Viewport::with_physical_size(
            Size::new(size.width, size.height),
            self.window.scale_factor(),
        );
        self.viewport = viewport;
    }

    pub fn resize(&mut self) -> Result<()> {
        self.sc = create_swap_chain(&self.window, &self.device, &self.surface)?;
        Ok(())
    }
}

struct Frontend {
    pub cursor_position: PhysicalPosition<f64>,
    pub modifiers: ModifiersState,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    debug: Debug,
    pub renderer: IcedRenderer,
    pub state: program::State<UserInterface>,
}

impl Frontend {
    pub fn new(glob: &GlobalState, ui: UserInterface) -> Self {
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
            ui,
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

    // update UI on window event, the event changes are stored in glob
    pub fn update(&mut self, glob: &GlobalState) -> Result<()> {
        self.state.update(
            glob.viewport.logical_size(),
            conversion::cursor_position(self.cursor_position, glob.viewport.scale_factor()),
            None,
            &mut self.renderer,
            &mut self.debug,
        );
        Ok(())
    }

    pub fn render(
        &mut self, glob: &GlobalState, target: &wgpu::SwapChainTexture, backend: &Backend
    ) -> Result<()> {
        let mut encoder = backend.renderer.create_encoder(Some("UI Encoder"), &glob.device);
        let mouse_interaction = self.renderer.backend_mut().draw(
            &glob.device,
            &mut self.staging_belt,
            &mut encoder,
            &target.view,
            &glob.viewport,
            self.state.primitive(),
            &self.debug.overlay(),
        );
        // submit the work
        self.staging_belt.finish();
        glob.queue.submit(Some(encoder.finish()));
        // recall staging buffers
        self.local_pool
            .spawner()
            .spawn(self.staging_belt.recall())
            .expect("Recall staging buffers");
        self.local_pool.run_until_stalled();
        // update the mouse cursor
        glob.window
            .set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));
        Ok(())
    }
}

struct Backend {
    pub settings: render::RenderSettings,
    pub renderer: render::Renderer,
}

impl Backend {
    pub fn new(glob: &GlobalState) -> Self {
        let settings = render::RenderSettings::new(glob.get_size());
        let renderer = render::Renderer::new(&glob.device, &glob.queue, &settings).unwrap();

        Self { settings, renderer }
    }

    pub fn update(&mut self, glob: &GlobalState) -> Result<()> {
        self.renderer.update(&glob.device, &glob.queue);
        Ok(())
    }

    pub fn render(&mut self, glob: &GlobalState, target: &wgpu::SwapChainTexture, frontend: &Frontend) -> Result<()> {
        // update render settings from frontend
        // todo more useful changes
        self.settings.clear_color = Some(frontend.state.program().background_color());
        // resize to actual current window size
        self.renderer.state.resize(glob.get_size(), &glob.device).context("Resize 3D renderer")?;
        // render using updated settings
        self
        .renderer
        .render_external(
            target,
            &self.settings,
            &glob.device,
            &glob.queue,
        )
    }
}
