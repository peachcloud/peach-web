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
extern crate tera;
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
use crate::structs::{
    FlashContext, JsonResponse, NetworkAddContext, NetworkContext, NetworkDetailContext, WiFi,
};
use crate::ws::*;

use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form};
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

// WEB PAGE ROUTES

//  [GET]       /                               Home
//  [GET]       /network                        Network overview
//  [GET]       /network/wifi/add               Add WiFi form
//  [GET]       /network/wifi/add?<ssid>        Add WiFi form (SSID populated)
//  [POST]      /network/wifi/add               Add WiFi handler
//  [GET]       /network/wifi                   List of networks
//  [GET]       /network/wifi?<ssid>            Details of single network

#[get("/")]
fn index() -> &'static str {
    "PeachCloud"
}

#[get("/network")]
fn network_card(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_card", &context)
}

#[get("/network/wifi/add")]
fn network_add(flash: Option<FlashMessage>) -> Template {
    let mut context = NetworkContext::build();
    // check to see if there is a flash message to display
    context.back = Some("/network/wifi".to_string());
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_add", &context)
}

#[get("/network/wifi/add?<ssid>")]
fn network_add_ssid(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
    let mut context = NetworkAddContext {
        back: Some("/network/wifi".to_string()),
        selected: Some(ssid.to_string()),
        flash_name: None,
        flash_msg: None,
    };
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_add", &context)
}

#[post("/network/wifi/add", data = "<wifi>")]
fn add_credentials(wifi: Form<WiFi>) -> Template {
    // generate and write wifi config to wpa_supplicant
    let ssid = wifi.ssid.to_string();
    let pass = wifi.pass.to_string();
    let add = network_add_wifi(ssid, pass);
    match add {
        Ok(_) => {
            debug!("Added WiFi credentials to wpa_supplicant config file.");
            match network_activate_client() {
                Ok(_) => debug!("Activated WiFi client on wlan0 interface."),
                Err(_) => warn!("Failed to activate WiFi client on wlan0 interface."),
            }
            // run RECONFIGURE to force reread of wpa_supplicant config
            // wpa_supplicant needs to be running, that's why we activate the
            // wireless client first before reconfiguration
            match network_reconfigure_wifi() {
                Ok(_) => debug!("Reread wpa_supplicant configuration from file."),
                Err(_) => warn!("Failed to force reread of wpa_supplicant configuration from file."),
            }
            // assign context through context_builder call
            let mut context = NetworkContext::build();
            context.flash_name = Some("success".to_string());
            context.flash_msg = Some("Added WiFi credentials".to_string());
            // render network_card view
            Template::render("network_card", &context)
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            let context = FlashContext {
                flash_name: Some("error".to_string()),
                flash_msg: Some("Failed to add WiFi credentials".to_string()),
            };
            // render network_card view
            Template::render("network_add", &context)
        }
    }
}

#[get("/network/wifi")]
fn network_list(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkContext::build();
    context.back = Some("/network".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_list", &context)
}

#[get("/network/wifi?<ssid>")]
fn network_detail(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkDetailContext::build();
    context.back = Some("/network/wifi".to_string());
    context.selected = Some(ssid.to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_detail", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// API ROUTES

//  [POST]       /api/v1/network/activate_ap
//  [POST]       /api/v1/network/activate_client
//  [GET]        /api/v1/network/ip
//  [GET]        /api/v1/network/rssi
//  [GET]        /api/v1/network/ssid
//  [GET]        /api/v1/network/state
//  [GET]        /api/v1/network/status
//  [GET]        /api/v1/network/wifi
//  [POST]       /api/v1/network/wifi
//  [POST]       /api/v1/network/wifi/forget         Forget network*
//  [POST]       /api/v1/network/wifi/modify         Modify network password*
//  [POST]       /api/v1/shutdown                    Shutdown device*

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

#[get("/api/v1/network/wifi")]
fn scan_networks() -> Json<JsonResponse> {
    // retrieve scan results for access-points within range of wlan0
    match network_scan_networks("wlan0".to_string()) {
        Ok(networks) => {
            let status = "success".to_string();
            let data = json!(networks);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Unable to scan for networks. Interface may be deactivated.".to_string();
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
                network_add,
                network_add_ssid,
                network_card,
                network_detail,
                network_list,
                activate_ap,
                activate_client,
                return_ip,
                return_rssi,
                return_ssid,
                return_state,
                return_status,
                scan_networks,
                add_wifi,
                add_credentials,
            ],
        )
        .register(catchers![not_found])
        .attach(Template::fairing())
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
