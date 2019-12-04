extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
extern crate serde_json;

use std::error;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug)]
pub enum NetworkError {
    NetworkHttp(jsonrpc_client_http::Error),
    NetworkClient(jsonrpc_client_core::Error),
    SerdeSerialize(serde_json::error::Error),
}

impl From<jsonrpc_client_http::Error> for NetworkError {
    fn from(err: jsonrpc_client_http::Error) -> NetworkError {
        NetworkError::NetworkHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for NetworkError {
    fn from(err: jsonrpc_client_core::Error) -> NetworkError {
        NetworkError::NetworkClient(err)
    }
}

impl From<serde_json::error::Error> for NetworkError {
    fn from(err: serde_json::error::Error) -> NetworkError {
        NetworkError::SerdeSerialize(err)
    }
}
