extern crate actix_web;
extern crate sonos;
extern crate regex;

#[macro_use]
extern crate serde;


use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::thread;
use regex::Regex;

mod models;
mod results;

use models::devices::Devices;
use results::track_info::TrackInfoResult;
use results::meta::Meta;
use results::error::Error;
use sonos::Speaker;
use std::rc::Rc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

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

#[get("/api/v1/{id}")]
async fn index(info: web::Path<(u32)>) -> impl Responder {
    format!("Hello {}!", info.into_inner())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("🎵  Starting Sonos-Web");
    println!("🔍  Searching for your SONOS system...");

    let devices = sonos::discover().unwrap();
    let s : Speaker;

    let f = HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8888")
        .unwrap()
        .start()
        .await;

    println!("Listening!");

    return f;
}


