use serde::Deserialize;

#[derive(Deserialize)]
pub enum Precision {
    F32,
    F64,
}

#[derive(Deserialize)]
pub enum RenderMode {
    RayTracer,
    RayMarcher,
}

#[derive(Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub samples: u32,
    pub max_depth: u32,
    pub precision: Precision,
    pub renderer: RenderMode,
    pub show_progress: bool,
}

#[derive(Deserialize)]
pub struct ImageConfig {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub gamma_correction: bool,
}
