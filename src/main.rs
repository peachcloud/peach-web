#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
mod tests;

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

use rocket::request::Form;
use rocket::response::NamedFile;

use rocket_contrib::json::{Json, JsonValue};

use jsonrpc_client_http::HttpTransport;

use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

// struct for handling wifi credentials
#[derive(FromForm)]
struct WiFi {
    ssid: String,
    pass: String,
}

// struct for interface address data
#[derive(Serialize)]
struct InterfaceAddresses {
    ap0: String,
    wlan0: String,
}

// struct for json response objects
#[derive(Serialize)]
struct JsonResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
}

// jsonrpc client
jsonrpc_client!(pub struct PeachNetworkClient {
    // returns the ip address for the given interface
    pub fn get_ip(&mut self, iface: String) -> RpcRequest<String>;

    // returns the ssid for the connected wifi network
    pub fn get_ssid(&mut self) -> RpcRequest<String>;

    // generates wpa_passphrase for given creds and writes to
    //  wpa_supplicant.conf
    pub fn add_wifi(&mut self, ssid: String, pass: String) -> RpcRequest<String>;

    // run ap / client-mode configuration script
    pub fn if_checker(&mut self) -> RpcRequest<String>;

    // take the given network interface down
    pub fn if_down(&mut self, iface: String) -> RpcRequest<String>;

    // bring the given network interface up
    pub fn if_up(&mut self, iface: String) -> RpcRequest<String>;
});

fn build_json_response(status: String, data: Option<String>, msg: Option<String>) -> JsonResponse {
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
    // create http transport for jsonrpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://127.0.0.1:3030/").unwrap();
    let mut client = PeachNetworkClient::new(transport_handle);

    // retrieve ip for wlan0 or set to x.x.x.x if not found
    let wlan_ip = client.get_ip("wlan0".to_string()).call();
    let wlan_ip = match wlan_ip {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };

    // retrieve ip for ap0 or set to x.x.x.x if not found
    let ap_ip = client.get_ip("ap0".to_string()).call();
    let ap_ip = match ap_ip {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };

    let ips = InterfaceAddresses {
        wlan0: wlan_ip,
        ap0: ap_ip,
    };

    let status: String = "success".to_string();
    let data = serde_json::to_string(&ips).unwrap();

    Json(build_json_response(status, Some(data), None))
}

#[get("/ssid")]
fn return_ssid() -> Json<JsonResponse> {
    // create http transport for jsonrpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://127.0.0.1:3030/").unwrap();
    let mut client = PeachNetworkClient::new(transport_handle);

    // retrieve ssid for connected network
    let ssid = client.get_ssid().call();
    let ssid = match ssid {
        Ok(network) => network,
        Err(_) => "Not currently connected".to_string(),
    };

    let status: String = "success".to_string();
    let data: String = ssid;

    Json(build_json_response(status, Some(data), None))
}

#[post("/wifi_credentials", data = "<wifi>")]
fn wifi_creds(wifi: Form<WiFi>) -> Json<JsonResponse> {
    // create http transport for jsonrpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://127.0.0.1:3030/").unwrap();
    let mut client = PeachNetworkClient::new(transport_handle);

    // generate and write wifi config to wpa_supplicant
    let ssid: String = wifi.ssid.to_string();
    let pass: String = wifi.pass.to_string();
    // this passage is a little sketchy but it works
    //  probably needs better handling of errors (ie. no unwraps)
    //  will panic if ifdown, ifup or ifchecker commands fail for some reason
    let add = client.add_wifi(ssid, pass).call();
    match add {
        Ok(_) => {
            let _ifdown = client.if_down("wlan0".to_string()).call().unwrap();
            let _ifup = client.if_up("wlan0".to_string()).call().unwrap();
            let _ifchecker = client.if_checker().call().unwrap();
            // json response for successful update
            let status: String = "success".to_string();
            let data: String = "WiFi credentials added".to_string();
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
            routes![index, files, wifi_creds, return_ip, return_ssid],
        )
        .register(catchers![not_found])
}

fn main() {
    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        rocket().launch();
    });

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
