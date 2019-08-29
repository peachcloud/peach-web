extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

#[derive(Debug)]
pub enum NetworkError {
    NetworkHttp(jsonrpc_client_http::Error),
    NetworkClient(jsonrpc_client_core::Error),
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
