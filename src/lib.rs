mod client;
mod ui;

pub use client::MediaClient;
pub use ui::run;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Settings {
    server: String,
    music: String,
    size: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: "127.0.0.1:6600".to_string(),
            music: "~/Music/".to_string(),
            size: 30,
        }
    }
}
