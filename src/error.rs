extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use std::{error, io};

use snafu::Snafu;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum NetworkError {
    #[snafu(display("Failed to call RPC client method: {}", source))]
    CallRpc { source: jsonrpc_client_core::Error },
    
    #[snafu(display("HTTP transport error: {}", source))]
    HttpTransport { source: jsonrpc_client_http::Error },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum WebsocketError {
    #[snafu(display("Failed to bind websocket server: {}", source))]
    BindAddress { source: io::Error },
}
