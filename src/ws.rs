use std::io;
use std::thread;

use websocket::sync::Server;
use websocket::{Message, OwnedMessage};

pub fn websocket_server(address: String) -> io::Result<()> {
    // Start listening for WebSocket connections
    let ws_server = Server::bind(address)?;

    info!("Listening for WebSocket connections.");
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

            debug!("Websocket connection from {}.", client_ip);

            let msg_text = "Websocket successfully connected.".to_string();
            let message = Message::text(msg_text);
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        debug!("Received close message.");
                        let message = Message::close();
                        sender.send_message(&message).unwrap();
                        debug!("Websocket client {} disconnected.", client_ip);
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        debug!("Received ping message.");
                        let message = Message::pong(data);
                        sender.send_message(&message).unwrap();
                    }
                    _ => {
                        sender.send_message(&message).unwrap();
                        debug!("Received unknown message: {:?}", message);
                    }
                }
            }
        });
    }

    Ok(())
}
