#![feature(proc_macro_hygiene, decl_macro)]

extern crate get_if_addrs;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate websocket;

mod error;
mod network;
mod structs;
#[cfg(test)]
mod tests;
mod ws;

use std::path::{Path, PathBuf};
use std::{env, thread};

use crate::error::BoxError;
use crate::network::*;
use crate::structs::{JsonResponse, WiFi};
use crate::ws::*;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};

// WEB PAGE ROUTES

#[get("/")]
fn index() -> &'static str {
    "PeachCloud"
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// API ROUTES

//  /api/v1/network/activate_ap
//  /api/v1/network/activate_client
//  /api/v1/network/ip
//  /api/v1/network/rssi
//  /api/v1/network/ssid
//  /api/v1/network/state
//  /api/v1/network/status
//  /api/v1/network/wifi

#[post("/api/v1/network/activate_ap")]
fn activate_ap() -> Json<JsonResponse> {
    // activate the wireless access point
    debug!("Activating WiFi access point.");
    match network_activate_ap() {
        Ok(_) => {
            let status = "success".to_string();
            Json(build_json_response(status, None, None))
        }
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to activate WiFi access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/activate_client")]
fn activate_client() -> Json<JsonResponse> {
    // activate the wireless client
    debug!("Activating WiFi client mode.");
    match network_activate_client() {
        Ok(_) => {
            let status = "success".to_string();
            Json(build_json_response(status, None, None))
        }
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to activate WiFi client mode.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/ip")]
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

    let status = "success".to_string();

    Json(build_json_response(status, Some(data), None))
}

#[get("/api/v1/network/rssi")]
fn return_rssi() -> Json<JsonResponse> {
    // retrieve rssi for connected network
    match network_get_rssi("wlan0".to_string()) {
        Ok(rssi) => {
            let status = "success".to_string();
            let data = json!(rssi);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/ssid")]
fn return_ssid() -> Json<JsonResponse> {
    // retrieve ssid for connected network
    match network_get_ssid("wlan0".to_string()) {
        Ok(network) => {
            let status = "success".to_string();
            let data = json!(network);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/state")]
fn return_state() -> Json<JsonResponse> {
    // retrieve state of wlan0 or set to x.x.x.x if not found
    let wlan_state = match network_get_state("wlan0".to_string()) {
        Ok(state) => state,
        Err(_) => "unavailable".to_string(),
    };
    // retrieve state for ap0 or set to x.x.x.x if not found
    let ap_state = match network_get_state("ap0".to_string()) {
        Ok(state) => state,
        Err(_) => "unavailable".to_string(),
    };
    let data = json!({
        "wlan0": wlan_state,
        "ap0": ap_state
    });

    let status = "success".to_string();

    Json(build_json_response(status, Some(data), None))
}

#[get("/api/v1/network/status")]
fn return_status() -> Json<JsonResponse> {
    // retrieve status info for wlan0 interface
    match network_get_status("wlan0".to_string()) {
        Ok(network) => {
            let status = "success".to_string();
            let data = json!(network);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi", data = "<wifi>")]
fn add_wifi(wifi: Form<WiFi>) -> Json<JsonResponse> {
    // generate and write wifi config to wpa_supplicant
    let ssid = wifi.ssid.to_string();
    let pass = wifi.pass.to_string();
    let add = network_add_wifi(ssid, pass);
    match add {
        Ok(_) => {
            debug!("Added WiFi credentials.");
            match network_reconnect_wifi("wlan0".to_string()) {
                Ok(_) => debug!("Reconnected wlan0 interface."),
                Err(_) => warn!("Failed to reconnect the wlan0 interface."),
            }
            // json response for successful update
            let status = "success".to_string();
            let data = json!("WiFi credentials added.");

            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            // json response for failed update
            let status = "error".to_string();
            let msg = "Failed to add WiFi credentials.".to_string();

            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// HELPER FUNCTIONS

fn build_json_response(
    status: String,
    data: Option<JsonValue>,
    msg: Option<String>,
) -> JsonResponse {
    JsonResponse { status, data, msg }
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
            routes![
                index,
                files,
                activate_ap,
                activate_client,
                return_ip,
                return_rssi,
                return_ssid,
                return_state,
                return_status,
                add_wifi
            ],
        )
        .register(catchers![not_found])
}

pub fn run() -> Result<(), BoxError> {
    info!("Starting up.");

    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        info!("Launching Rocket server.");
        rocket().launch();
    });

    let ws_addr = env::var("PEACH_WEB_WS").unwrap_or_else(|_| "0.0.0.0:5115".to_string());
    match websocket_server(ws_addr) {
        Ok(_) => debug!("Websocket server terminated without error."),
        Err(e) => error!("Error starting the websocket server: {}", e),
    };

    Ok(())
}
