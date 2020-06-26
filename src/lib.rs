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
mod routes;
mod stats;
mod structs;
#[cfg(test)]
mod tests;
mod ws;

use std::{env, thread};

use crate::error::BoxError;
use crate::json_api::*;
use crate::routes::*;
use crate::ws::*;

use rocket_contrib::templates::Template;

// create rocket instance & mount web & json routes (makes testing easier)
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
                modify_password,         // WEB ROUTE
                network_add_ssid,        // WEB ROUTE
                network_add_wifi,        // WEB ROUTE
                network_card,            // WEB ROUTE
                network_detail,          // WEB ROUTE
                network_list,            // WEB ROUTE
                network_modify_password, // WEB ROUTE
                reboot_cmd,              // WEB ROUTE
                shutdown_cmd,            // WEB ROUTE
                shutdown_menu,           // WEB ROUTE
                activate_ap,             // JSON API
                activate_client,         // JSON API
                add_wifi,                // JSON API
                connect_ap,              // JSON API
                disconnect_ap,           // JSON API
                forget_ap,               // JSON API
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
        .register(catchers![not_found, internal_error])
        .attach(Template::fairing())
}

// launch the rocket server
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
