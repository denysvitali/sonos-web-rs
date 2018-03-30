use models::track::Track;
use results::meta::Meta;

#[derive(Serialize)]
pub struct TrackInfoResult{
    pub track: Option<Track>,
    pub meta: Meta
}