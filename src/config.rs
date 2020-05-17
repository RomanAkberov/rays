use serde::Deserialize;
use crate::{
    ray_tracer::RayTracerConfig,
};

#[derive(Deserialize)]
pub struct Config {
    pub image: ImageConfig,
    pub ray_tracer: RayTracerConfig,
}

#[derive(Deserialize)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    pub gamma_correction: bool,
}
