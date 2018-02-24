#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate rocket_contrib;

extern crate sonos;
extern crate rocket;

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

fn main() {
    println!("🎵  Starting Sonos-Web");
    println!("🔍  Searching for your SONOS system...");

    let devices = sonos::discover().unwrap();
    
    rocket::ignite()
        .mount("/", routes![index, play, pause, set_volume])
        .manage(Devices { speakers: devices})
        .launch();
}
