use core::time::Duration;

#[derive(Serialize)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_art: String,
    pub uri: String,
    pub duration: Duration,
    pub running_time: Duration
}