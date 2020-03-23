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
    FlashContext, JsonResponse, NetworkAddContext, NetworkContext, NetworkDetailContext,
    NetworkListContext, Ssid, WiFi,
};
use crate::ws::*;

use percent_encoding::percent_decode;

use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;

// WEB PAGE ROUTES

//  [GET]       /                               Home
//  [GET]       /network/ap/activate            Activate WiFi access point mode
//  [GET]       /network                        Network overview
//  [GET]       /network/wifi                   List of networks
//  [GET]       /network/wifi?<ssid>            Details of single network
//  [GET]       /network/wifi/activate          Activate WiFi client mode
//  [GET]       /network/wifi/add               Add WiFi form
//  [POST]      /network/wifi/add               WiFi form submission
//  [GET]       /network/wifi/add?<ssid>        Add WiFi form (SSID populated)
//  [POST]      /network/wifi/forget            Remove WiFi
//  [POST]      /network/wifi/modify            Modify network password

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

#[get("/network/wifi")]
fn network_list(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkListContext::build();
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
    // decode ssid from url
    let decoded_ssid = percent_decode(ssid.as_bytes()).decode_utf8().unwrap();
    context.selected = Some(decoded_ssid.to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_detail", &context)
}

#[get("/network/wifi/add")]
fn network_add(flash: Option<FlashMessage>) -> Template {
    let mut context = NetworkContext::build();
    // set back icon link to network route
    context.back = Some("/network".to_string());
    // check to see if there is a flash message to display
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
    // decode ssid from url
    let decoded_ssid = percent_decode(ssid.as_bytes()).decode_utf8().unwrap();
    let mut context = NetworkAddContext {
        back: Some("/network/wifi".to_string()),
        selected: Some(decoded_ssid.to_string()),
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
    let ssid = &wifi.ssid;
    let pass = wifi.pass.to_string();
    let ssid_copy = ssid.to_string();
    let add = network_add_wifi(ssid_copy, pass);
    match add {
        Ok(_) => {
            debug!("Added WiFi credentials to wpa_supplicant config file.");
            // run RECONFIGURE to force reread of wpa_supplicant config
            // wpa_supplicant needs to be running
            // if it's not, we catch the error and activate client mode
            match network_reconfigure_wifi() {
                Ok(_) => {
                    debug!("Reread wpa_supplicant configuration from file.");
                    match network_get_id("wlan0", ssid) {
                        Ok(id) => match network_select_network(id, "wlan0".to_string()) {
                            Ok(_) => debug!("Selected chosen network."),
                            Err(_) => warn!("Failed to select chosen network."),
                        },
                        Err(_) => warn!("Failed to retrieve the network ID."),
                    }
                }
                Err(_) => {
                    warn!("Failed to force reread of wpa_supplicant configuration from file.");
                    match network_activate_client() {
                        Ok(_) => debug!("Activated WiFi client."),
                        Err(_) => warn!("Failed to activate WiFi client."),
                    }
                }
            }
            let context = FlashContext {
                flash_name: Some("success".to_string()),
                flash_msg: Some("Added WiFi credentials".to_string()),
            };
            Template::render("network_add", &context)
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            let context = FlashContext {
                flash_name: Some("error".to_string()),
                flash_msg: Some("Failed to add WiFi credentials".to_string()),
            };
            Template::render("network_add", &context)
        }
    }
}

#[post("/network/wifi/forget", data = "<network>")]
fn forget_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let iface = "wlan0";
    let ssid = &network.ssid;
    //let iface_copy = iface.to_string();
    //let ssid_copy = ssid.to_string();
    debug!("Fetching ID for given interface and SSID");
    let url = uri!(network_detail: ssid);
    match forget_network(iface, ssid) {
        Ok(msg) => {
            Flash::success(Redirect::to(url), msg)
        }
        Err(e) => Flash::error(Redirect::to(url), e)
    }
    /*
    match network_get_id(iface, ssid) {
        Ok(id) => match network_remove_wifi(id.as_str(), iface) {
            Ok(_) => {
                debug!("WiFi credentials removed for chosen network.");
                match network_save_config() {
                    Ok(_) => {
                        let url = uri!(network_detail: ssid);
                        Flash::success(Redirect::to(url), "Removed WiFi credentials.")
                    }
                    Err(_) => remove_wifi_failed(ssid),
                }
            }
            Err(_) => remove_wifi_failed(ssid),
        },
        Err(_) => remove_wifi_failed(ssid),
    }
    */
}

#[post("/network/wifi/modify", data = "<wifi>")]
fn modify_password(wifi: Form<WiFi>) -> Flash<Redirect> {
    let iface = "wlan0";
    let ssid = &wifi.ssid;
    let pass = &wifi.pass;
    //let iface_copy = iface.to_string();
    //let ssid_copy = ssid.to_string();
    match network_get_id(iface, ssid) {
        Ok(id) => match network_new_password(id.as_str(), iface, pass) {
            Ok(_) => {
                debug!("WiFi password updated for chosen network.");
                match network_save_config() {
                    Ok(_) => {
                        debug!("WiFi configuration saved.");
                        let url = format!("/network/wifi?={}", ssid);
                        Flash::success(Redirect::to(url), "Updated WiFi password.")
                    }
                    Err(_) => {
                        warn!("Failed to save WiFi configuration updates.");
                        let url = format!("/network/wifi?ssid={}", ssid);
                        Flash::error(
                            Redirect::to(url),
                            "Failed to save WiFi configuration updates.",
                        )
                    }
                }
            }
            Err(_) => {
                warn!("Failed to update WiFi password.");
                let url = format!("/network/wifi?ssid={}", ssid);
                Flash::error(Redirect::to(url), "Failed to update WiFi password.")
            }
        },
        Err(_) => {
            warn!("Failed to retrieve the network ID for given SSID.");
            let url = format!("/network/wifi?ssid={}", ssid);
            Flash::error(Redirect::to(url), "Failed to retrieve the network ID.")
        }
    }
}

#[get("/network/ap/activate")]
fn deploy_ap() -> Flash<Redirect> {
    // activate the wireless access point
    debug!("Activating WiFi access point.");
    match network_activate_ap() {
        Ok(_) => Flash::success(Redirect::to("/network"), "Activated WiFi access point."),
        Err(_) => Flash::error(
            Redirect::to("/network"),
            "Failed to activate WiFi access point.",
        ),
    }
}

#[get("/network/wifi/activate")]
fn deploy_client() -> Flash<Redirect> {
    // activate the wireless client
    debug!("Activating WiFi client mode.");
    match network_activate_client() {
        Ok(_) => Flash::success(Redirect::to("/network"), "Activated WiFi client."),
        Err(_) => Flash::error(Redirect::to("/network"), "Failed to activate WiFi client."),
    }
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
//  [POST]       /api/v1/network/wifi/forget         Forget / remove network
//  [POST]       /api/v1/network/wifi/modify         Modify network password*
//  [GET]        /api/v1/ping
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

#[post("/api/v1/network/wifi/forget", data = "<network>")]
fn remove_wifi(network: Form<Ssid>) -> Json<JsonResponse> {
    //let iface = "wlan0".to_string();
    //let ssid_copy = network.ssid.to_string();
    let ssid = &network.ssid;
    let iface = "wlan0";
    match forget_network(ssid, iface) {
        Ok(msg) => {
            let status = "success".to_string();
            Json(build_json_response(status, None, Some(msg)))
        },
        Err(e) => {
            let status = "error".to_string();
            Json(build_json_response(status, None, Some(e)))
        }
    }
    /*
    match network_get_id(iface, ssid_copy) {
        Ok(id) => {
            match network_remove_wifi(id, ssid.to_string()) {
                Ok(_) => {
                    debug!("Removed chosen network.");
                    // json response for successful update
                    let status = "success".to_string();
                    let msg = "WiFi credentials removed.".to_string();
                    Json(build_json_response(status, None, Some(msg)))
                }
                Err(_) => {
                    warn!("Failed to remove chosen network.");
                    // json response for failed update
                    let status = "error".to_string();
                    let msg = "Failed to remove WiFi credentials.".to_string();
                    Json(build_json_response(status, None, Some(msg)))
                }
            }
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            // json response for failed update
            let status = "error".to_string();
            let msg = "Failed to add WiFi credentials.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
    */
}

#[post("/api/v1/network/wifi/modify", data = "<wifi>")]
fn new_password(wifi: Form<WiFi>) -> Json<JsonResponse> {
    let iface = "wlan0";
    let ssid = &wifi.ssid;
    let pass = &wifi.pass;
    //let iface_copy = iface.to_string();
    //let ssid_copy = ssid.to_string();
    match network_get_id(iface, ssid) {
        Ok(id) => match network_new_password(id.as_str(), iface, pass) {
            Ok(_) => {
                debug!("WiFi password updated for chosen network.");
                match network_save_config() {
                    Ok(_) => {
                        debug!("WiFi configuration saved.");
                        // json response for successful update
                        let status = "success".to_string();
                        let msg = "WiFi password updated.".to_string();
                        Json(build_json_response(status, None, Some(msg)))
                    }
                    Err(_) => {
                        warn!("Failed to save WiFi configuration updates.");
                        let status = "error".to_string();
                        let msg = "Failed to save WiFi configuration updates.".to_string();
                        Json(build_json_response(status, None, Some(msg)))
                    }
                }
            }
            Err(_) => {
                warn!("Failed to update WiFi password.");
                let status = "error".to_string();
                let msg = "Failed to update WiFi password.".to_string();
                Json(build_json_response(status, None, Some(msg)))
            }
        },
        Err(_) => {
            warn!("Failed to retrieve the network ID for given SSID.");
            let status = "error".to_string();
            let msg = "Failed to retrieve network ID.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// status route: useful for checking connectivity from web client
#[get("/api/v1/ping")]
fn ping_pong() -> Json<JsonResponse> {
    // ping pong
    let status = "success".to_string();
    let msg = "pong!".to_string();

    Json(build_json_response(status, None, Some(msg)))
}

// HELPER FUNCTIONS

fn build_json_response(
    status: String,
    data: Option<JsonValue>,
    msg: Option<String>,
) -> JsonResponse {
    JsonResponse { status, data, msg }
}

/*
fn remove_wifi_failed(ssid: &String) -> Flash<Redirect> {
    warn!("Failed to get ID for chosen network.");
    let url = uri!(network_detail: ssid);
    Flash::error(Redirect::to(url), "Failed to remove WiFi credentials.")
}
*/

// let's create a helper function which returns a result type
fn forget_network(iface: &str, ssid: &str) -> Result<String, String> {
    debug!("Fetching ID for given interface and SSID");
    //let iface_copy = &iface;
    //let ssid_copy = &ssid;
    match network_get_id(iface, ssid) {
        Ok(id) => match network_remove_wifi(id.as_str(), iface) {
            Ok(_) => {
                debug!("WiFi credentials removed for chosen network.");
                match network_save_config() {
                    Ok(_) => Ok("Network configuration updated.".to_string()),
                    Err(_) => Err("Failed to save network configuration.".to_string())
                }
            }
            Err(_) => Err("Failed to remove configuration for given ID and iface.".to_string()),
        },
        Err(_) => Err("Failed to get ID for given iface and SSID.".to_string()),
    }
}

#[catch(404)]
fn not_found() -> Template {
    debug!("404 Page Not Found");
    // HACK: this is just here to satisfy the context requirement
    let context = FlashContext {
        flash_name: Some("error".to_string()),
        flash_msg: Some("No resource found for given URL".to_string()),
    };
    Template::render("not_found", context)
}

// create rocket instance & mount routes (makes testing easier)
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,            // WEB ROUTE
                files,            // WEB ROUTE
                add_credentials,  // WEB ROUTE
                deploy_ap,        // WEB ROUTE
                deploy_client,    // WEB ROUTE
                forget_wifi,      // WEB ROUTE
                modify_password,  // WEB ROUTE
                network_add,      // WEB ROUTE
                network_add_ssid, // WEB ROUTE
                network_card,     // WEB ROUTE
                network_detail,   // WEB ROUTE
                network_list,     // WEB ROUTE
                activate_ap,      // JSON API
                activate_client,  // JSON API
                new_password,     // JSON API
                return_ip,        // JSON API
                return_rssi,      // JSON API
                return_ssid,      // JSON API
                return_state,     // JSON API
                return_status,    // JSON API
                scan_networks,    // JSON API
                add_wifi,         // JSON API
                remove_wifi,      // JSON API
                ping_pong,        // JSON API
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
