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

mod device;
mod error;
mod json_api;
mod network;
mod oled;
mod stats;
mod structs;
#[cfg(test)]
mod tests;
mod ws;

use std::path::{Path, PathBuf};
use std::{env, thread};

use crate::device::*;
use crate::error::BoxError;
use crate::json_api::*;
use crate::network::*;
use crate::structs::{
    DeviceContext, FlashContext, NetworkAddContext, NetworkContext, NetworkDetailContext,
    NetworkListContext, Ssid, WiFi,
};
use crate::ws::*;

use percent_encoding::percent_decode;

use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket_contrib::templates::Template;

// WEB PAGE ROUTES

//  [GET]       /                               Home
//  [GET]       /device                         Device statistics
//  [GET]       /device/reboot                  Reboot device
//  [GET]       /device/shutdown                Shutdown device
//  [GET]       /network/ap/activate            Activate WiFi access point mode
//  [GET]       /network                        Network overview
//  [GET]       /network/wifi                   List of networks
//  [GET]       /network/wifi?<ssid>            Details of single network
//  [GET]       /network/wifi/activate          Activate WiFi client mode
//  [GET]       /network/wifi/add               Add WiFi form
//  [POST]      /network/wifi/add               WiFi form submission
//  [GET]       /network/wifi/add?<ssid>        Add WiFi form (SSID populated)
//  [POST]      /network/wifi/connect           Connect to WiFi access point
//  [POST]      /network/wifi/forget            Remove WiFi*
//  [GET]       /network/wifi/modify?<ssid>     Modify WiFi password form
//  [POST]      /network/wifi/modify            Modify network password*
//  [GET]       /shutdown                       Shutdown menu
//
//  * not yet working 100%

#[get("/")]
fn index() -> Template {
    let context = FlashContext {
        flash_name: None,
        flash_msg: None,
    };
    Template::render("index", &context)
}

#[get("/device")]
fn device_stats(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = DeviceContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("device", &context)
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
fn network_add_wifi(flash: Option<FlashMessage>) -> Template {
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
    match network_add(&wifi.ssid, &wifi.pass) {
        Ok(_) => {
            debug!("Added WiFi credentials.");
            // force reread of wpa_supplicant.conf file with new credentials
            match network_reconfigure() {
                Ok(_) => debug!("Successfully reconfigured wpa_supplicant."),
                Err(_) => warn!("Failed to reconfigure wpa_supplicant."),
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

#[post("/network/wifi/connect", data = "<network>")]
fn connect_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let ssid = &network.ssid;
    let url = uri!(network_detail: ssid);
    match network_id("wlan0", &ssid) {
        Ok(id) => match network_connect(&id, "wlan0") {
            Ok(_) => Flash::success(Redirect::to(url), "Connected to chosen network."),
            Err(_) => Flash::error(Redirect::to(url), "Failed to connect to chosen network."),
        },
        Err(_) => Flash::error(Redirect::to(url), "Failed to retrieve the network ID."),
    }
}

#[get("/network/wifi/disconnect")]
fn disconnect_wifi() -> Flash<Redirect> {
    let url = uri!(network_card);
    match network_disconnect("wlan0") {
        Ok(_) => Flash::success(Redirect::to(url), "Disconnected from WiFi network."),
        Err(_) => Flash::error(Redirect::to(url), "Failed to disconnect from WiFi network."),
    }
}

#[post("/network/wifi/forget", data = "<network>")]
fn forget_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let ssid = &network.ssid;
    let url = uri!(network_detail: ssid);
    match forget_network("wlan0", &ssid) {
        Ok(msg) => Flash::success(Redirect::to(url), msg),
        Err(_) => Flash::error(
            Redirect::to(url),
            "Failed to remove WiFi credentials".to_string(),
        ),
    }
}

#[get("/network/wifi/modify?<ssid>")]
fn network_modify_password(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
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
    Template::render("network_modify", &context)
}

#[post("/network/wifi/modify", data = "<wifi>")]
fn modify_password(wifi: Form<WiFi>) -> Flash<Redirect> {
    let ssid = &wifi.ssid;
    let pass = &wifi.pass;
    let url = uri!(network_detail: ssid);
    match update_password("wlan0", ssid, pass) {
        Ok(msg) => Flash::success(Redirect::to(url), msg),
        Err(_) => Flash::error(
            Redirect::to(url),
            "Failed to update WiFi password".to_string(),
        ),
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

#[get("/shutdown")]
fn shutdown_menu(flash: Option<FlashMessage>) -> Template {
    let mut context = FlashContext {
        flash_name: None,
        flash_msg: None,
    };
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("shutdown", &context)
}

#[get("/device/reboot")]
fn reboot_cmd() -> Flash<Redirect> {
    match device_reboot() {
        Ok(_) => Flash::success(Redirect::to("/shutdown"), "Rebooting the device."),
        Err(_) => Flash::error(Redirect::to("/shutdown"), "Failed to reboot the device."),
    }
}

#[get("/device/shutdown")]
fn shutdown_cmd() -> Flash<Redirect> {
    match device_shutdown() {
        Ok(_) => Flash::success(Redirect::to("/shutdown"), "Shutting down the device."),
        Err(_) => Flash::error(Redirect::to("/shutdown"), "Failed to shutdown the device."),
    }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
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
                index,                   // WEB ROUTE
                files,                   // WEB ROUTE
                add_credentials,         // WEB ROUTE
                connect_wifi,            // WEB ROUTE
                disconnect_wifi,         // WEB ROUTE
                deploy_ap,               // WEB ROUTE
                deploy_client,           // WEB ROUTE
                device_stats,            // WEB ROUTE
                forget_wifi,             // WEB ROUTE
                network_modify_password, // WEB ROUTE
                modify_password,         // WEB ROUTE
                network_add_ssid,        // WEB ROUTE
                network_add_wifi,        // WEB ROUTE
                network_card,            // WEB ROUTE
                network_detail,          // WEB ROUTE
                network_list,            // WEB ROUTE
                reboot_cmd,              // WEB ROUTE
                shutdown_cmd,            // WEB ROUTE
                shutdown_menu,           // WEB ROUTE
                activate_ap,             // JSON API
                activate_client,         // JSON API
                add_wifi,                // JSON API
                connect_ap,              // JSON API
                disconnect_ap,           // JSON API
                new_password,            // JSON API
                ping_pong,               // JSON API
                ping_network,            // JSON API
                ping_oled,               // JSON API
                ping_stats,              // JSON API
                return_ip,               // JSON API
                return_rssi,             // JSON API
                return_ssid,             // JSON API
                return_state,            // JSON API
                return_status,           // JSON API
                reboot_device,           // JSON API
                scan_networks,           // JSON API
                shutdown_device,         // JSON API
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
