#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate sonos;
extern crate rocket;
extern crate ws;
extern crate core;

use rocket::State;
use rocket_contrib::Json;
use rocket::response::NamedFile;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::thread;

mod models;
mod results;

use models::devices::Devices;
use results::track_info::TrackInfoResult;
use results::meta::Meta;
use models::track::Track;
use results::error::Error;
use ws::Message;
use sonos::Speaker;
use std::rc::Rc;

macro_rules! speaker_check {
    ($a:expr, $b:expr) => {
        if !speaker_exists($a, $b){
            return Json(json!({
                "success": false,
                "error": "Speaker doesn't exists"
            }))
        }
    };
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/public/index.html")
}

fn speaker_exists(devices: &State<Devices>, id: usize)->bool {
    id < devices.speakers.len()
}

#[get("/api/<id>/volume/<value>")]
fn set_volume(devices: State<Devices>, id: usize, value: usize) -> Json {
    speaker_check!(&devices, id);
    
    let result = devices.speakers[id as usize].set_volume(value as u8);
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/play")]
fn play(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].play();
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/pause")]
fn pause(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].pause();
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/next")]
fn next(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].next();
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/prev")]
fn prev(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].previous();
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/mute")]
fn mute(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].mute();
    Json(json!({
        "success": result.is_ok()
    }))
}

#[get("/api/<id>/unmute")]
fn unmute(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].unmute();
    Json(json!({
        "success": result.is_ok()
    }))
}


#[get("/api/<id>/track_info")]
fn track_info(devices: State<Devices>, id: usize) -> Json<TrackInfoResult> {
    if !speaker_exists(&devices, id) {
        return Json(TrackInfoResult{
            track: None,
            meta: Meta {
                success: false,
                error: Some(Error {
                    code: 1,
                    message: String::from("Unable to find this device")
                })
            }
        });
    }

    let result = devices.speakers[id as usize].track().unwrap();
    Json(TrackInfoResult{
        meta: Meta {
            success: true,
            error: None
        },
        track: Some(Track {
            title: result.title,
            artist: result.artist,
            album: result.album,
            album_art: result.album_art,
            uri: result.uri,
            duration: result.duration,
            running_time: result.running_time
        })
    })
}

#[get("/api/<id>/transport_state")]
fn transport_state(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].transport_state();

    match result {
        Err(_e)=> {
            Json(json!({
                "success": false,
                "transport_state": serde_json::Value::Null
            }))
        },
        Ok(transport_state) =>{
            Json(json!({
                "success": true,
                "transport_state": format!("{:?}", transport_state)
            }))
        }
    }
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open(Path::new("static/public/").join(file)).ok()
}

fn main() {
    println!("🎵  Starting Sonos-Web");
    println!("🔍  Searching for your SONOS system...");

    let devices = sonos::discover().unwrap();
    let s : Speaker;

    let devices_d = devices.clone();

    thread::spawn(move || {
        ws::listen("127.0.0.1:3012", |out| {
            let d = devices.clone();
            move |msg : Message | {
                match msg.as_text()? {
                    "status" => {
                        let result = d[0].track().unwrap();
                        let json = serde_json::to_string(&TrackInfoResult{
                            meta: Meta {
                                success: true,
                                error: None
                            },
                            track: Some(Track {
                                title: result.title,
                                artist: result.artist,
                                album: result.album,
                                album_art: result.album_art,
                                uri: result.uri,
                                duration: result.duration,
                                running_time: result.running_time
                            })
                        }).unwrap();
                        out.send(json);
                    }
                    _ => {
                        println!("Unrecognized");
                    }
                };
                //println!("Received {}", message);
                out.send(msg)
            }
        }).unwrap();
    });
    
    rocket::ignite()
        .mount("/", routes![ index, play, pause, next, prev, mute, unmute, track_info, set_volume, files])
        .manage(Devices { speakers: devices_d})
        .launch();

}
