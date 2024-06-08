use chill_mp::MediaClient;
use figment::providers::{Format, Serialized};
use figment::{providers::Toml, Figment};
use serde::{Deserialize, Serialize};
use viuer::Config;

#[derive(Deserialize, Serialize)]
struct Settings {
    server: String,
    music: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: "127.0.0.1:6600".to_string(),
            music: "~/Music/".to_string(),
        }
    }
}

fn main() -> Result<(), figment::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("chill").unwrap();
    let settings: Settings = Figment::from(Serialized::defaults(Settings::default()))
        .merge(Toml::file(xdg.get_config_file("mp.toml")))
        .extract()?;
    let mut client = MediaClient::new(&settings.server);
    let config = Config {
        x: 20,
        y: 20,
        width: Some(50),
        height: Some(50),
        ..Default::default()
    };
    match client.title() {
        None => println!("No Song Playing."),
        Some(song) => println!("Now Playing: {}", song),
    }
    match client.playtime() {
        None => (),
        Some(playtime) => println!("{} -> {}", playtime.elapsed(), playtime.duration()),
    }
    match client.art(settings.music) {
        None => (),
        Some(image) => drop(viuer::print(&image, &config).expect("Failed to print image.")),
    };
    Ok(())
}
