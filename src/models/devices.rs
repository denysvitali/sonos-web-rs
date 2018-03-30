use sonos::Speaker;

#[derive(Serialize)]
pub struct Devices  {
    pub speakers: Vec<Speaker>
}