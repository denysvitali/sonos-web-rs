use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct Speaker {
    pub ip: IpAddr,
    pub model: String,
    pub model_number: String,
    pub software_version: String,
    pub hardware_version: String,
    pub serial_number: String,
    pub name: String,
    pub uuid: String,
}

pub fn from_sonos(speaker: &sonos::Speaker) -> Speaker {
    Speaker {
        ip: IpAddr::from(match speaker.ip {
            IpAddr::V4(i) => {
                IpAddr::from(i)
            },
            IpAddr::V6(i) => {
                IpAddr::from(i)
            }
        }),
        model: String::from(&speaker.model),
        model_number: String::from(&speaker.model_number),
        software_version: String::from(&speaker.software_version),
        hardware_version: String::from(&speaker.hardware_version),
        serial_number: String::from(&speaker.serial_number),
        name: String::from(&speaker.name),
        uuid: String::from(&speaker.uuid),
    }
}