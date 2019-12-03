use sonos::Track;
use crate::results::meta::Meta;

pub struct TrackInfoResult{
    pub track: Option<Track>,
    pub meta: Meta
}