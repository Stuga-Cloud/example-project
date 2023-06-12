use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub log_level: String,
    pub server_port: u16,
}

impl Settings {
    fn new() -> Self {
        Self {
            environment: "dev".to_string(),
            log_level: "debug".to_string(),
            server_port: 8080,
        }
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}
