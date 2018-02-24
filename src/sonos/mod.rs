extern crate ssdp;

use std::io::{self, Read};
use std::thread;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::net::SocketAddr;

use self::ssdp::FieldMap;
use self::ssdp::header::{HeaderMut, HeaderRef, Man, MX, NT, NTS, USN, ST};
use self::ssdp::message::{NotifyListener, NotifyMessage, SearchRequest, Listen, Multicast};


mod device;

use self::device::SonosDevice;

fn utf8_to_string(bytes: &[Vec<u8>]) -> String {
    let a = bytes.get(0).unwrap();
    String::from_utf8(a.clone()).unwrap()
}

fn parse_sonos(src: &SocketAddr, message: &NotifyMessage) -> SonosDevice {
    let ip = src.to_string();
    let household = utf8_to_string(message.get_raw("X-RINCON-HOUSEHOLD").unwrap());
    let wifiMode = utf8_to_string(message.get_raw("X-RINCON-WIFIMODE").unwrap()) == "1";
    let server = utf8_to_string(message.get_raw("SERVER").unwrap());
    let variant = utf8_to_string(message.get_raw("X-RINCON-VARIANT").unwrap()).parse::<i32>().unwrap();
    let sd = SonosDevice::new(
        &ip, &household, wifiMode, &server, variant
    );
    sd
}

pub fn discover() -> Vec<SonosDevice> {
    let mut foundDevices : Vec<SonosDevice> = Vec::new();
    let (sender, receiver) = channel();

    thread::spawn(move || {
        for (msg, src) in NotifyListener::listen().unwrap() {
            //println!("Received The Following Message From {}:\n{:?}\n", src, msg);
            let nt = match msg.get_raw("NT") {
                    Some(nt) => utf8_to_string(nt),
                    _ => String::new()
            };
            if nt == "urn:smartspeaker-audio:service:SpeakerGroup:1" {
                println!("Found a SONOS speaker group!\n{:?}",msg);
            }
            else if nt == "urn:schemas-upnp-org:device:ZonePlayer:1" {
                println!("Found a Sonos ZP at {}", src);
                //sender.send(parse_sonos(&src, &msg));
            }
            else if nt.starts_with("uuid:RINCON_") {
                println!("May be a SONOS device: {}", nt);
            }
            else {
                println!("Not a SONOS system: {}", nt);
            }
        }
    });

    // Create A Test Message

    // Set Some Headers
    let mut request = SearchRequest::new();


    // Set Some Headers
    request.set(Man);
    request.set(MX(1));
    request.set(ST::Target(FieldMap::URN(String::from("schemas-upnp-org:device:ZonePlayer:1"))));
    request.set(ST::All);

    request.multicast().unwrap();
    
    match receiver.recv() {
        Ok((msg,src)) => { 
            println!("{:?}", msg);
            foundDevices.push(parse_sonos(src, msg));
            
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    };
    

    foundDevices
}
