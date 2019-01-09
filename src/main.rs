#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::prelude::*;

use rocket::response::NamedFile;
use rocket::request::Form;

// struct for handling wifi credentials
#[derive(FromForm)]
struct WiFi {
    ssid: String,
    pass: String,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/wifi_credentials", data = "<wifi>")]
fn wifi_creds(wifi: Form<WiFi>) -> String {

    // generate configuration based on provided ssid & password
    let output = Command::new("wpa_passphrase")
        .arg(&wifi.ssid)
        .arg(&wifi.pass)
        .stdout(Stdio::piped())
        .output().unwrap_or_else(|e| {
            panic!("Failed to execute wpa_passphrase command: {}", e)
    });

    let wpa_details = &*(output.stdout);

    // append wpa_passphrase output to wpa_supplicant.conf if successful
    if output.status.success() {
        let file = OpenOptions::new()
            .append(true)
            .open("/etc/wpa_supplicant/wpa_supplicant.conf");
        
        let _file = match file {
            Ok(mut f) => f.write(wpa_details),
            Err(_) => {
                panic!("There was a problem appending to the file")
            }
        };
        // change: format as json response in production
        format!("{}", "WiFi credentials added. Attempting connection...")
    } else {
        // change: format as json response in production
        format!("{}", "Failed to add WiFi credentials")
    }
    // change: redirect to / (index)
}

fn main() {
    rocket::ignite().mount("/", routes![index, files, wifi_creds]).launch();
}
