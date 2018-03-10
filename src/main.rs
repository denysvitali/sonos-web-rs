#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate rocket_contrib;

extern crate sonos;
extern crate rocket;
extern crate serde_json;

use rocket::State;
use rocket_contrib::Json;

struct Devices {
    speakers: Vec<sonos::Speaker>
}

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
fn index() -> &'static str {
    "Hello, world! 🙂"
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
fn track_info(devices: State<Devices>, id: usize) -> Json {
    speaker_check!(&devices, id);

    let result = devices.speakers[id as usize].track().unwrap();
    Json(json!({
        "success": true,
        "track": {
            "title": result.title,
            "artist": result.artist,
            "album": result.album,
            "albumArt": result.albumArt,
            "uri": result.uri,
            "duration": result.duration,
            "running_time": result.running_time
        }
    }))
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

fn main() {
    println!("🎵  Starting Sonos-Web");
    println!("🔍  Searching for your SONOS system...");

    let devices = sonos::discover().unwrap();
    
    rocket::ignite()
        .mount("/", routes![index, play, pause, next, prev, mute, unmute, track_info, set_volume])
        .manage(Devices { speakers: devices})
        .launch();
}
