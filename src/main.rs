#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod sonos;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! 🙂"
}

fn main() {
    println!("🎵  Starting Sonos-Web");
    //rocket::ignite().mount("/", routes![index]).launch();

    println!("🔍  Searching for your SONOS system...");
    let devices = sonos::discover();
    println!("{:?}", devices);
}
