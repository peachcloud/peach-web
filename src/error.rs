extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use std::error;

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
/*
impl From<jsonrpc_client_http::Error> for NetworkError {
    fn from(err: jsonrpc_client_http::Error) -> NetworkError {
        NetworkError::NetworkHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for NetworkError {
    fn from(err: jsonrpc_client_core::Error) -> NetworkError {
        NetworkError::NetworkClient(err)
    }
}*/
