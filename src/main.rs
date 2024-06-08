use chill_mp::MediaClient;

fn main() {
    let mut client = MediaClient::new("127.0.0.1:6600");
    match client.title() {
        None => println!("No Song Playing."),
        Some(song) => println!("Now Playing: {}", song),
    }
    match client.playtime() {
        None => (),
        Some(playtime) => println!("{} -> {}", playtime.elapsed(), playtime.duration()),
    }
}
