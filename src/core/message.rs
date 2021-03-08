#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
    LoadProject,
    LoadProjectChange(String), // save asset, etc
}
