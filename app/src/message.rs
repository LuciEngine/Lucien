#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Tick,
    LoadProject,
    LoadProjectChange(String), // save asset, etc
}
