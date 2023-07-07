use std::env;

use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::SETTINGS;

pub fn setup() {
    if env::var_os("RUST_LOG").is_none() {
        let level = SETTINGS.log_level.as_str();
        let env = format!("rustapi={level},tower_http={level}");

        env::set_var("RUST_LOG", env);
    }
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .init();
}
