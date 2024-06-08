use mpd::Client;
use std::ops::Div;
use std::time::Duration;

/// `MediaClient` acts as the MPD client and interacts with MPD.
pub struct MediaClient {
    conn: Client,
}

/// `PlayTime` encapsulates the duration and elapsed time of a song.
pub struct PlayTime {
    elapsed: Duration,
    duration: Duration,
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
            "{}:{}",
            self.elapsed.as_secs().div(60),
            self.elapsed.as_secs() % 60
        )
    }
    /// Gets the duration in a formatted string.
    pub fn duration(&self) -> String {
        format!(
            "{}:{}",
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

    pub fn title(&mut self) -> Option<String> {
        return match self.conn.currentsong() {
            Err(_) => None,
            Ok(song) => match song {
                None => None,
                Some(music) => Some(music.title.unwrap_or("No Song Name".to_string())),
            },
        };
    }

    pub fn playtime(&mut self) -> Option<PlayTime> {
        return match self.conn.status() {
            Err(_) => None,
            Ok(status) => status.time.map(|time| time.into()),
        };
    }
}
