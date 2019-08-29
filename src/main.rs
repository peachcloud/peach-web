#![feature(proc_macro_hygiene, decl_macro)]

mod error;
mod network;
mod structs;
#[cfg(test)]
mod tests;

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

use std::io;
use std::path::{Path, PathBuf};
use std::thread;

use crate::network::*;
use crate::structs::WiFi;

//use jsonrpc_client_http::HttpTransport;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

// struct for json response objects
#[derive(Serialize)]
struct JsonResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
}

fn build_json_response(status: String, data: Option<JsonValue>, msg: Option<String>) -> JsonResponse {
    JsonResponse {
        status: status,
        data: data,
        msg: msg,
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
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

#[post("/add_wifi", data = "<wifi>")]
fn add_wifi(wifi: Form<WiFi>) -> Json<JsonResponse> {
    // generate and write wifi config to wpa_supplicant
    let ssid: String = wifi.ssid.to_string();
    let pass: String = wifi.pass.to_string();
    // this passage is a little sketchy but it works
    //  probably needs better handling of errors (ie. no unwraps)
    //  will panic if ifdown, ifup or ifchecker commands fail for some reason
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

fn main() {
    // initialize the logger
    env_logger::init();

    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        rocket().launch();
    });

    // -> move to websocket.rs and call with server address:port
    // Start listening for WebSocket connections
    let ws_server = Server::bind("0.0.0.0:2794").unwrap();

    for connection in ws_server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            if !connection
                .protocols()
                .contains(&"rust-websocket".to_string())
            {
                connection.reject().unwrap();
                return;
            }

            let mut client = connection
                .use_protocol("rust-websocket")
                .accept()
                .unwrap();

            let client_ip = client.peer_addr().unwrap();

            // -> replace with info!(format!("Connection from {}", client_ip));
            println!("Connection from {}", client_ip);

            let msg_text = "Websocket successfully connected".to_string();
            let message = Message::text(msg_text);
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = Message::close();
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", client_ip);
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        let message = Message::pong(data);
                        sender.send_message(&message).unwrap();
                    }
                    _ => {
                        sender.send_message(&message).unwrap();
                        println!("{:?}", message);
                    }
                }
            }
        });
    }
}
