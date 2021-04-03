use crate::{Frontend, GlobalState};
use anyhow::{Context, Result};
use iced_wgpu::wgpu;
use lucien_render as render;

pub(crate) struct Backend {
    pub settings: render::RenderSettings,
    pub renderer: render::Renderer,
}

impl Backend {
    pub fn new(glob: &GlobalState) -> Result<Self> {
        let settings = render::RenderSettings::new(glob.get_size());
        let renderer = render::Renderer::new(&glob.device, &glob.queue, &settings)
            .context("Failed to create 3D renderer")?;

        Ok(Self { settings, renderer })
    }

    pub fn update(&mut self, glob: &GlobalState) -> Result<()> {
        self.renderer.update(&glob.device, &glob.queue);
        Ok(())
    }

    pub fn render(
        &mut self, glob: &GlobalState, target: &wgpu::SwapChainTexture, frontend: &Frontend,
    ) -> Result<()> {
        // update render settings from frontend
        // todo more useful changes
        self.settings.clear_color = Some(frontend.state.program().background_color());
        // resize to actual current window size
        self.renderer
            .state
            .resize(glob.get_size(), &glob.device)
            .context("Resize 3D renderer")?;
        // render using updated settings
        self.renderer
            .render_external(target, &self.settings, &glob.device, &glob.queue)
    }
}
