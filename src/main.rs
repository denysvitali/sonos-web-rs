extern crate actix_web;
extern crate sonos;
extern crate regex;
extern crate env_logger;

#[macro_use]
extern crate serde;


use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::thread;
use regex::Regex;

use serde_json::json;

mod models;
mod results;

use models::devices::Devices;
use models::speaker;
use results::track_info::TrackInfoResult;
use results::meta::Meta;
use results::error::Error;
use std::rc::Rc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, post, HttpRequest, middleware};
use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::web::BytesMut;
use crate::models::speaker::Speaker;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use crate::models::response::volume::VolumeResponse;

macro_rules! speaker_check {
    ($a:expr, $b:expr) => {
        if !speaker_exists($a, $b){
            return HttpResponse::NotFound().json(json!({
                "success": false,
                "error": "Speaker doesn't exists"
            }))
        }
    };
}

fn speaker_exists(uuid: &str, speakers: &Vec<Speaker>) -> bool {
    match get_speaker_by_uuid(uuid, speakers) {
        Some(s) => true,
        None => false
    }
}

struct WebAppState {
    pub speakers: Mutex<Vec<sonos::Speaker>>
}

fn get_speaker_by_uuid<'a>(uuid: &str, speakers: &'a Vec<Speaker>) -> Option<&'a Speaker> {
    for speaker in speakers.iter() {
        if &speaker.uuid == uuid {
            return Some(speaker);
        }
    }

    None
}

#[get("/api/v1/speakers")]
async fn get_speakers(state: web::Data<WebAppState>, req: HttpRequest) -> HttpResponse {
    let sonos_speakers : &Vec<sonos::Speaker> = &*state.speakers.lock().unwrap();
    let speakers : Vec<Speaker> = sonos_speakers.iter().map(speaker::from_sonos).collect();
    HttpResponse::Ok().json(&speakers)
}

#[get("/api/v1/speakers/{uuid}")]
async fn get_speaker(state: web::Data<WebAppState>, uuid: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let sonos_speakers : &Vec<sonos::Speaker> = &*state.speakers.lock().unwrap();
    let speakers : Vec<Speaker> = sonos_speakers.iter().map(speaker::from_sonos).collect();

    let uuid_str = uuid.into_inner();
    speaker_check!(&uuid_str, &speakers);

    match get_speaker_by_uuid(uuid_str.as_str(), &speakers) {
        Some(s) => {
            return HttpResponse::Ok().json(s)
        }
        None => {
            return HttpResponse::NotFound().body("");
        }
    }
}

#[post("/api/v1/speakers/{uuid}/volume/{value}")]
async fn set_speaker_volume(state: web::Data<WebAppState>,
                            path: web::Path<(String,u8)>,
                            req: HttpRequest) -> HttpResponse {
    let sonos_speakers : &Vec<sonos::Speaker> = &*state.speakers.lock().unwrap();
    let speakers : Vec<Speaker> = sonos_speakers.iter().map(speaker::from_sonos).collect();

    let inner = path.into_inner();
    speaker_check!(inner.0.as_ref(), &speakers);

    let volume = inner.1;
    if volume < 0 || volume > 100 {
        return HttpResponse::BadRequest().body("");
    }

    match get_speaker_by_uuid(inner.0.as_ref(), &speakers) {
        Some(s) => {
            let sonos_speaker = sonos::Speaker::from_ip(s.ip);
            match sonos_speaker {
                Ok(speaker) => {
                    speaker.set_volume(volume);
                    return HttpResponse::Ok().body("");
                }
                Err(e) => {
                    return HttpResponse::InternalServerError().body("");
                }
            }
        }
        None => {
            return HttpResponse::NotFound().body("");
        }
    }
}

#[get("/api/v1/speakers/{uuid}/volume")]
async fn get_speaker_volume(state: web::Data<WebAppState>,
                            path: web::Path<String>,
                            req: HttpRequest) -> HttpResponse {
    let sonos_speakers : &Vec<sonos::Speaker> = &*state.speakers.lock().unwrap();
    let speakers : Vec<Speaker> = sonos_speakers.iter().map(speaker::from_sonos).collect();

    let inner = path.into_inner();
    speaker_check!(inner.as_ref(), &speakers);

    match get_speaker_by_uuid(inner.as_ref(), &speakers) {
        Some(s) => {
            let sonos_speaker = sonos::Speaker::from_ip(s.ip);
            match sonos_speaker {
                Ok(speaker) => {
                    match speaker.volume() {
                        Ok(v) => {
                            return HttpResponse::Ok().json(VolumeResponse{
                                volume: v
                            })
                        },
                        Err(e) => {
                            return HttpResponse::InternalServerError().body("");
                        }
                    }
                    return HttpResponse::Ok().body("");
                }
                Err(e) => {
                    return HttpResponse::InternalServerError().body("");
                }
            }
        }
        None => {
            return HttpResponse::NotFound().body("");
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("🎵  Starting Sonos-Web");
    println!("🔍  Searching for your SONOS system...");

    let mut devices = sonos::discover().unwrap();
    println!("Listening on 127.0.0.1:8888");

    let mut app_state = web::Data::new(
        WebAppState{
            speakers: Mutex::new(devices)
        }
    );

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let f = HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .service(get_speakers)
            .service(set_speaker_volume)
            .service(get_speaker_volume)
            .service(get_speaker)
        )
        .bind("127.0.0.1:8888")
        .unwrap()
        .run()
        .await;

    return f;
}


