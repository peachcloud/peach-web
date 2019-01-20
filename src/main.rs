#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate websocket;
extern crate get_if_addrs;

use std::io;
use std::thread;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::prelude::*;

use rocket::response::NamedFile;
use rocket::request::Form;

use rocket_contrib::json::Json;

use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

// struct for handling wifi credentials
#[derive(FromForm)]
struct WiFi {
    ssid: String,
    pass: String,
}

// struct for json config update responses
#[derive(Serialize)]
struct ConfigUpdate {
    status: String,
    msg: String,
}

// struct for json interface address responses
#[derive(Serialize)]
struct InterfaceAddresses {
    ap0: String,
    wlan0: String,
}

// retrieve ip address for specified interface
fn get_ip(iface: String) -> Option<String> {
    let ifaces = get_if_addrs::get_if_addrs().unwrap();
    ifaces
        .iter()
        .find(|&i| i.name == iface)
        .map(|iface| iface.ip().to_string())
}

fn build_json_response(status: String, msg: String) -> ConfigUpdate {
    ConfigUpdate {
        status: status,
        msg: msg,
    }
}

fn json_ip_response(ap0: String, wlan0: String) -> InterfaceAddresses {
    InterfaceAddresses {
        ap0: ap0,
        wlan0: wlan0,
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
fn return_ip() -> Json<InterfaceAddresses> {
    // retrieve ip for wlan0 or set to x.x.x.x if not found
    let wlan_ip = get_ip("wlan0".to_string());
    let wlan_ip = match wlan_ip {
        Some(ip) => ip,
        None => "x.x.x.x".to_string(),
    };
    
    // retrieve ip for ap0 or set to x.x.x.x if not found
    let ap_ip = get_ip("ap0".to_string());
    let ap_ip = match ap_ip {
        Some(ip) => ip,
        None => "x.x.x.x".to_string(),
    };
    
    Json(json_ip_response(ap_ip, wlan_ip))
}

#[post("/wifi_credentials", data = "<wifi>")]
fn wifi_creds(wifi: Form<WiFi>) -> Json<ConfigUpdate> {

    // generate configuration based on provided ssid & password
    let output = Command::new("wpa_passphrase")
        .arg(&wifi.ssid)
        .arg(&wifi.pass)
        .stdout(Stdio::piped())
        .output().unwrap_or_else(|e| {
            panic!("Failed to execute wpa_passphrase command: {}", e)
    });

    let wpa_details = &*(output.stdout);

    // append wpa_passphrase output to wpa_supplicant.conf if successful
    if output.status.success() {
        // open file in append mode
        let file = OpenOptions::new()
            .append(true)
            .open("/etc/wpa_supplicant/wpa_supplicant.conf");
        
        let _file = match file {
            // if file exists & open succeeds, write wifi configuration
            Ok(mut f) => f.write(wpa_details),
            // need to handle this better: create file if not found
            //  and seed with 'ctrl_interace' & 'update_config' settings
            Err(_) => panic!("There was a problem appending to the file")
        };
        
        let if_down = Command::new("sudo")
            .arg("/sbin/ifdown")
            .arg("wlan0")
            .output().unwrap_or_else(|e| {
                panic!("Failed to execute ifdown command: {}", e)
            });

        if if_down.status.success() {
            println!("wlan0 down");
        } else { println!("wlan0 down failed"); };
        
        let if_up = Command::new("sudo")
            .arg("/sbin/ifup")
            .arg("wlan0")
            .output().unwrap_or_else(|e| {
                panic!("Failed to execute ifup command: {}", e)
            });

        if if_up.status.success() {
            println!("wlan0 up");
        } else { println!("wlan0 up failed"); };

        let iface_checker = Command::new("sudo")
            .arg("/bin/bash")
            .arg("/home/glyph/interface_checker.sh")
            .output().unwrap_or_else(|e| {
                panic!("Failed to execute interface_checker command: {}", e)
            });

        // json response for successful update
        let status : String = "Success".to_string();
        let msg : String = "WiFi credentials added. Attempting connection."
            .to_string();
        Json(build_json_response(status, msg))
    } else {
        // json response for failed update
        let status : String = "Fail".to_string();
        let msg : String = "Failed to add WiFi credentials.".to_string();
        Json(build_json_response(status, msg))
    }
}

fn main() {
    
    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        rocket::ignite()
            .mount("/", routes![index, files, wifi_creds, return_ip])
            .launch();
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
                .use_protocol("rust-websocket").accept().unwrap();

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
