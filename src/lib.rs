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

use std::{env, thread};
use std::path::{Path, PathBuf};

use crate::error::BoxError;
use crate::network::*;
use crate::structs::{NetworkContext, JsonResponse, WiFi};
use crate::ws::*;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

// WEB PAGE ROUTES

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkContext::build();
    context.connect_wifi = false;
    // check to see if there is a flash message to be displayed
    match flash {
        Some(flash) => {
            // add flash message contents to the context object
            context.flash_name = Some(flash.name().to_string());
            context.flash_msg = Some(flash.msg().to_string());
        },
        _ => (),
    };
    // template_dir is set in Rocket.toml
    Template::render("index", &context)
}

#[get("/connect_to_wifi")]
fn connect_wifi() -> Template {
    let mut context = NetworkContext::build();
    // set flag to display Connect to WiFi form
    context.connect_wifi = true;
    context.flash_name = None;
    context.flash_msg = None;
    Template::render("connect_wifi_form", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// API ROUTES

#[post("/api/activate_ap")] 
fn activate_ap() -> Flash<Redirect> {
    // activate the wireless access point
    debug!("Activating WiFi access point.");
    // TODO: do we really want to redirect here?
    // redirect will fail if AP is not up yet
    // might be better not to have a flash message and
    // allow manual refresh instead (maybe with in-page button)
    match network_activate_ap() {
        Ok(_) => Flash::success(Redirect::to("/"), "Activated WiFi client mode."),
        Err(_) => Flash::error(Redirect::to("/"), "Failed to activate WiFi client mode."),
    }
}

#[post("/api/activate_client")] 
fn activate_client() -> Flash<Redirect> {
    // activate the wireless client
    debug!("Activating WiFi client mode.");
    match network_activate_client() {
        Ok(_) => Flash::success(Redirect::to("/"), "Activated WiFi client mode."),
        Err(_) => Flash::error(Redirect::to("/"), "Failed to activate WiFi client mode."),
    }
}

#[post("/api/add_wifi", data = "<wifi>")]
fn add_wifi(wifi: Form<WiFi>) -> Json<JsonResponse> {
    // generate and write wifi config to wpa_supplicant
    let ssid: String = wifi.ssid.to_string();
    let pass: String = wifi.pass.to_string();
    let add = network_add_wifi(ssid, pass);
    match add {
        Ok(_) => {
            debug!("Added WiFi credentials.");
            match network_reconnect_wifi("wlan0".to_string()) {
                Ok(_) => debug!("Reconnected wlan0 interface."),
                Err(_) => warn!("Failed to reconnect the wlan0 interface."),
            }
            // json response for successful update
            let status: String = "success".to_string();
            let data = json!("WiFi credentials added");
            
            return Json(build_json_response(status, Some(data), None));
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            // json response for failed update
            let status: String = "error".to_string();
            let msg: String = "Failed to add WiFi credentials".to_string();
            
            return Json(build_json_response(status, None, Some(msg)));
        }
    };
}

#[get("/api/ip")]
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

#[get("/api/ssid")]
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
        status,
        data,
        msg,
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
            routes![index, files, activate_ap, activate_client, add_wifi, connect_wifi, return_ip, return_ssid],
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
        Err(e) => error!("Error starting the websocket server: {}", e)
    };

    Ok(())
}
