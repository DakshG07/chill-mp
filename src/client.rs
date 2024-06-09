use id3::Tag;
use image::DynamicImage;
use mpd::Client;
use std::ops::Div;
use std::time::Duration;

/// `MediaClient` acts as the MPD client and interacts with MPD.
pub struct MediaClient {
    conn: Client,
}

/// `PlayTime` encapsulates the duration and elapsed time of a song.
pub struct PlayTime {
    pub elapsed: Duration,
    pub duration: Duration,
}

impl From<(Duration, Duration)> for PlayTime {
    /// Construct a Playtime from a tuple of elapsed and total time.
    fn from((elapsed, duration): (Duration, Duration)) -> Self {
        Self { elapsed, duration }
    }
}

impl PlayTime {
    /// Gets the elapsed time in a formatted string.
    pub fn elapsed(&self) -> String {
        format!(
            "{}:{:02}",
            self.elapsed.as_secs().div(60),
            self.elapsed.as_secs() % 60
        )
    }
    /// Gets the duration in a formatted string.
    pub fn duration(&self) -> String {
        format!(
            "{}:{:02}",
            self.duration.as_secs().div(60),
            self.duration.as_secs() % 60
        )
    }
}

impl MediaClient {
    /// Construct a new `MediaClient` listening on the given server address.
    pub fn new(server: &str) -> Self {
        Self {
            conn: Client::connect(server).unwrap(),
        }
    }

    /// Retrieves the title of the currently playing song.
    pub fn title(&mut self) -> Option<String> {
        if let Ok(song) = self.conn.currentsong() {
            return Some(song?.title.unwrap_or("No Artist".to_string()));
        };
        None
    }

    /// Retrieves the title of the currently playing song.
    pub fn artist(&mut self) -> Option<String> {
        if let Ok(song) = self.conn.currentsong() {
            return Some(song?.artist.unwrap_or("No Artist".to_string()));
        };
        None
    }

    /// Retrieves the playtime of the currently playing song, in the form of a `PlayTime`.
    pub fn playtime(&mut self) -> Option<PlayTime> {
        self.conn.status().ok()?.time.map(|time| time.into())
    }

    /// Gets the filename of the current song.
    pub fn file(&mut self) -> Option<String> {
        Some(self.conn.currentsong().ok()??.file)
    }

    /// Retrieves the album art of the currently playing song, as a `DynamicImage`.
    pub fn art(&mut self, music_dir: &str) -> Option<DynamicImage> {
        let filename = self.file()?;
        let path = format!("{}{}", music_dir, filename);
        let tag = Tag::read_from_path(path).ok()?;
        let picture = tag.pictures().next()?;
        image::load_from_memory(&picture.data).ok()
    }
}
