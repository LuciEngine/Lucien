use anyhow::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Tick,
    UpdateComplete(Result<(), Error>),
    LoadProject,
    LoadProjectChange(String), // save asset, etc
}
