use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
}
