use chill_mp::{run, Settings};
use figment::providers::{Format, Serialized};
use figment::{providers::Toml, Figment};

fn main() -> Result<(), figment::Error> {
    let xdg = xdg::BaseDirectories::with_prefix("chill").unwrap();
    let settings: Settings = Figment::from(Serialized::defaults(Settings::default()))
        .merge(Toml::file(xdg.get_config_file("mp.toml")))
        .extract()?;
    run(settings).unwrap();
    Ok(())
}
