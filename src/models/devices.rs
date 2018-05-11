use sonos::Speaker;
use std::rc::Rc;

#[derive(Serialize)]
pub struct Devices  {
    pub speakers: Vec<Speaker>
}