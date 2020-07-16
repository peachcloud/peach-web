//! # peach-web
//!
//! `peach-web` provides a web interface for monitoring and interacting with the
//! PeachCloud device. This allows administration of the single-board computer
//! (ie. Raspberry Pi) running PeachCloud, as well as the ssb-server and related
//! plugins.
//!
//! ## Design
//!
//! `peach-web` is written primarily in Rust and presents a web interface for
//! interacting with the device. The stack currently consists of Rocket (Rust
//! web framework), Tera (Rust template engine inspired by Jinja2 and the Django
//! template language), HTML, CSS and JavaScript. Additional functionality is
//! provided by JSON-RPC clients for the `peach-network` and `peach-stats`
//! microservices.
//!
//! HTML is rendered server-side. Request handlers call JSON-RPC microservices
//! and serve HTML and assets. A JSON API is exposed for remote calls and
//! dynamic client-side content updates via vanilla JavaScript following
//! unobstructive design principles. A basic Websockets server is included,
//! though is not currently utilised. Each Tera template is passed a context
//! object. In the case of Rust, this object is a `struct` and must implement
//! `Serialize`. The fields of the context object are available in the context
//! of the template to be rendered.

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

pub mod context;
pub mod device;
pub mod error;
pub mod json_api;
pub mod network;
pub mod network_client;
pub mod oled_client;
pub mod routes;
pub mod stats_client;
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
                help,                    // WEB ROUTE
                messages,                // WEB ROUTE
                modify_password,         // WEB ROUTE
                network_add_ssid,        // WEB ROUTE
                network_add_wifi,        // WEB ROUTE
                network_card,            // WEB ROUTE
                network_detail,          // WEB ROUTE
                network_list,            // WEB ROUTE
                network_modify_password, // WEB ROUTE
                peers,                   // WEB ROUTE
                profile,                 // WEB ROUTE
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

    // NOTE: websockets are not currently in use (may be in the future)
    let ws_addr = env::var("PEACH_WEB_WS").unwrap_or_else(|_| "0.0.0.0:5115".to_string());
    match websocket_server(ws_addr) {
        Ok(_) => debug!("Websocket server terminated without error."),
        Err(e) => error!("Error starting the websocket server: {}", e),
    };

    Ok(())
}
