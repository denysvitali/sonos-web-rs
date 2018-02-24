#[derive(Debug)]
pub struct SonosDevice {
    IP: String,
    household: String,
    wifiMode: bool,
    server: String,
    variant: i32
}

impl SonosDevice {
    pub fn new(ip: &str, household: &str, wifiMode: bool, server: &str, variant: i32) -> SonosDevice {
        SonosDevice { IP: String::from(ip), household: String::from(household), wifiMode: wifiMode, server: String::from(server), variant: variant }
    }
}
