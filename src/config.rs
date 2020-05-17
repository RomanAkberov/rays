use serde::Deserialize;

#[derive(Deserialize)]
pub enum Precision {
    F32,
    F64,
}

#[derive(Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub samples: u32,
    pub max_depth: u32,
    pub precision: Precision,
}

#[derive(Deserialize)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    pub gamma_correction: bool,
}
