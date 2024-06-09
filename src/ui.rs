use crate::client::PlayTime;
use crate::{MediaClient, Settings};
use crossterm::style::Stylize;
use crossterm::{cursor, terminal, QueueableCommand};
use std::io::{self, Write};
use std::ops::Div;
use std::thread::sleep;
use std::time::Duration;
use viuer::Config;

/// Centers text to correct width.
fn center(width: u16, text: String) -> String {
    // Format literal can center text at desired width
    format!("{:^1$}", text, width as usize)
}
/// Generates and centers a timeline.
fn timeline(width: u16, playtime: PlayTime) -> String {
    let ratio = f32::div(
        playtime.elapsed.as_secs() as f32,
        playtime.duration.as_secs() as f32,
    );
    let length = 20;
    // Uses the same technique in center, but with left and right align
    center(
        width + 29, // Account for color codes
        format!(
            "{0} {1}{2} {3}",
            playtime.elapsed(),
            format!("{1:—>0$}", (ratio * (length as f32)) as usize, "-").green(),
            format!("{1:—>0$}", ((1.0 - ratio) * (length as f32)) as usize, "").dark_grey(),
            playtime.duration()
        ),
    )
}
pub fn run(settings: Settings) -> io::Result<()> {
    let mut client = MediaClient::new(&settings.server);
    let mut stdout = io::stdout();

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    let mut width = termsize::get().unwrap().cols;

    let mut config = Config {
        x: (width - settings.size).div(2),
        y: 5,
        width: Some(settings.size.into()),
        // Half this so that images look normal when rendered with half-block characters
        height: Some(settings.size.div(2).into()),
        ..Default::default()
    };
    let mut displayed_file = "".to_string();
    stdout.queue(cursor::Hide)?;
    loop {
        width = termsize::get().unwrap().cols;
        config.x = (width - settings.size).div(2);

        let current_file = client.file().unwrap_or("".to_string());
        if current_file != displayed_file {
            displayed_file = current_file;
            stdout.queue(terminal::Clear(terminal::ClearType::All))?;
            match client.art(settings.music.as_str()) {
                None => (),
                Some(image) => drop(viuer::print(&image, &config).expect("Failed to print image.")),
            };
            println!(
                "\n{}",
                match client.title() {
                    None => center(width, "No Song Playing.".to_string()).bold(),
                    Some(song) => center(width, song).bold(),
                }
            );
            // Save timeline position
            stdout.queue(cursor::SavePosition)?;
        } else {
            stdout.queue(cursor::RestorePosition)?;
        }
        println!(
            "\n{}",
            match client.playtime() {
                None => "".to_string().bold(),
                Some(playtime) => timeline(width, playtime).bold(),
            }
        );
        stdout.flush()?;
        sleep(Duration::from_millis(500));
    }
    #[allow(unreachable_code)]
    Ok(())
}
