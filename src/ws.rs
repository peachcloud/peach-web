use std::io;
use std::thread;

use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

pub fn websocket_server(address: String) -> io::Result<()> {
    // Start listening for WebSocket connections
    //let ws_server = Server::bind("0.0.0.0:2794").unwrap();
    let ws_server = Server::bind(address)?;

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

    Ok(())
}
