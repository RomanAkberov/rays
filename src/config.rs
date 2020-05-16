use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub max_depth: u32,
    pub gamma_correction: bool,
}
