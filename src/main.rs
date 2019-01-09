#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
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

use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

// struct for handling wifi credentials
#[derive(FromForm)]
struct WiFi {
    ssid: String,
    pass: String,
}

fn get_ip() {
    // List all of the machine's network interfaces
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        println!("{:#?}", iface);
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

#[post("/wifi_credentials", data = "<wifi>")]
fn wifi_creds(wifi: Form<WiFi>) -> String {

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
        let file = OpenOptions::new()
            .append(true)
            .open("/etc/wpa_supplicant/wpa_supplicant.conf");
        
        let _file = match file {
            Ok(mut f) => f.write(wpa_details),
            Err(_) => {
                panic!("There was a problem appending to the file")
            }
        };
        // change: format as json response in production
        format!("{}", "WiFi credentials added. Attempting connection...")
    } else {
        // change: format as json response in production
        format!("{}", "Failed to add WiFi credentials")
    }
    // change: redirect to / (index)
}

fn main() {
    
    // spawn a separate thread for rocket to prevent blocking websockets
    thread::spawn(|| {
        rocket::ignite()
            .mount("/", routes![index, files, wifi_creds])
            .launch();
    });

    get_ip();

    // Start listening for WebSocket connections
	let ws_server = Server::bind("127.0.0.1:2794").unwrap();

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

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = Message::text("Websocket connected.");
			client.send_message(&message).unwrap();

			let (mut receiver, mut sender) = client.split().unwrap();

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = Message::close();
						sender.send_message(&message).unwrap();
						println!("Client {} disconnected", ip);
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
