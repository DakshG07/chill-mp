use chill_mp::MediaClient;
use viuer::Config;

fn main() {
    let mut client = MediaClient::new("127.0.0.1:6600");
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
    match client.art() {
        None => (),
        Some(image) => drop(viuer::print(&image, &config).expect("Failed to print image.")),
    }
}
