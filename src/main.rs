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
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get, HttpRequest, middleware};
use serde::Serialize;
use actix_web::http::StatusCode;
use actix_web::web::BytesMut;
use crate::models::speaker::Speaker;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;

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

struct WebAppState {
    pub speakers: Mutex<Vec<sonos::Speaker>>
}

#[get("/api/v1/speakers")]
async fn get_speakers(state: web::Data<WebAppState>, req: HttpRequest) -> HttpResponse {
    let sonos_speakers : &Vec<sonos::Speaker> = &*state.speakers.lock().unwrap();
    let speakers : Vec<Speaker> = sonos_speakers.iter().map(speaker::from_sonos).collect();
    HttpResponse::Ok().json(&speakers)
}

#[get("/api/v1/hello/{id}")]
async fn index(info: web::Path<u32>) -> impl Responder {
    format!("Hello {}!", info.into_inner())
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
            .register_data(app_state.clone())
            .service(index)
            .service(get_speakers)
        )
        .bind("127.0.0.1:8888")
        .unwrap()
        .start()
        .await;

    return f;
}


