use crate::render::RgbaBuffer;
use anyhow::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Tick,
    SaveRenderResult,
    UpdateComplete(Result<(), Error>),
    RenderComplete(Result<RgbaBuffer, Error>),
    RenderSaveComplete(Result<(), Error>),
    LoadProject,
    LoadProjectChange(String), // save asset, etc
}
