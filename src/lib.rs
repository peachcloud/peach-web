#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate get_if_addrs;
extern crate jsonrpc_client_http;
extern crate websocket;

mod error;
mod network;
mod structs;
#[cfg(test)]
mod tests;
mod ws;

use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use crate::error::BoxError;
use crate::network::*;
use crate::structs::{JsonResponse, WiFi};
use crate::ws::*;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};

// WEB PAGE ROUTES

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// API ROUTES

#[post("/add_wifi", data = "<wifi>")]
fn add_wifi(wifi: Form<WiFi>) -> Json<JsonResponse> {
    // generate and write wifi config to wpa_supplicant
    let ssid: String = wifi.ssid.to_string();
    let pass: String = wifi.pass.to_string();
    let add = network_add_wifi(ssid, pass);
    match add {
        Ok(_) => {
            network_reconnect_wifi("wlan0".to_string()).expect("Failed to reconnect the wlan0 interface");
            // json response for successful update
            let status: String = "success".to_string();
            let data = json!("WiFi credentials added");
            return Json(build_json_response(status, Some(data), None));
        }
        Err(_) => {
            // json response for failed update
            let status: String = "error".to_string();
            let msg: String = "Failed to add WiFi credentials".to_string();
            return Json(build_json_response(status, None, Some(msg)));
        }
    };
}

#[get("/ip")]
fn return_ip() -> Json<JsonResponse> {
    // retrieve ip for wlan0 or set to x.x.x.x if not found
    let wlan_ip = match network_get_ip("wlan0".to_string()) {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };
    // retrieve ip for ap0 or set to x.x.x.x if not found
    let ap_ip = match network_get_ip("ap0".to_string()) {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };
    let data = json!({
        "wlan0": wlan_ip,
        "ap0": ap_ip
    });

    let status: String = "success".to_string();

    Json(build_json_response(status, Some(data), None))
}

#[get("/ssid")]
fn return_ssid() -> Json<JsonResponse> {
    // retrieve ssid for connected network
    let ssid = match network_get_ssid("wlan0".to_string()) {
        Ok(network) => network,
        Err(_) => "Not currently connected".to_string(),
    };
    let status: String = "success".to_string();
    let data = json!(ssid);

    Json(build_json_response(status, Some(data), None))
}

// HELPER FUNCTIONS

fn build_json_response(status: String, data: Option<JsonValue>, msg: Option<String>) -> JsonResponse {
    JsonResponse {
        status: status,
        data: data,
        msg: msg,
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "msg": "Resource was not found"
    })
}

// create rocket instance & mount routes (makes testing easier)
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![index, files, add_wifi, return_ip, return_ssid],
        )
        .register(catchers![not_found])
}

pub fn run() -> Result<(), BoxError> {
    // initialize the logger
    env_logger::init();

    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        rocket().launch();
    });

    // Start listening for WebSocket connections
    let ws_addr = "0.0.0.0:2794".to_string();
    match websocket_server(ws_addr) {
        Ok(_) => println!("All good"),
        Err(_) => println!("Error starting the websocket server")
    };

    Ok(())
}