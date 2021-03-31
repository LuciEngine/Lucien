use anyhow::Result;
use futures::task::SpawnExt;
use iced_native::program;
use iced_wgpu::Renderer;
use iced_winit::{conversion, futures, winit, Debug};

use crate::widgets::UserInterface;
use crate::{Backend, GlobalState};
use winit::{dpi::PhysicalPosition, event::ModifiersState};

pub(crate) struct Frontend {
    pub cursor_position: PhysicalPosition<f64>,
    pub modifiers: ModifiersState,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    debug: Debug,
    pub renderer: Renderer,
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
        let mut renderer = Renderer::new(Backend::new(&glob.device, Settings::default()));
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
        &mut self, glob: &GlobalState, target: &wgpu::SwapChainTexture, backend: &Backend,
    ) -> Result<()> {
        let mut encoder = backend
            .renderer
            .create_encoder(Some("UI Encoder"), &glob.device);
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
