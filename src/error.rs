//! Basic error handling for the network, OLED and stats JSON-RPC clients.

use std::error;

pub type BoxError = Box<dyn error::Error>;

#[derive(Debug)]
pub enum MenuError {
    OledHttp(jsonrpc_client_http::Error),
    OledClient(jsonrpc_client_core::Error),
}

impl From<jsonrpc_client_http::Error> for MenuError {
    fn from(err: jsonrpc_client_http::Error) -> MenuError {
        MenuError::OledHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for MenuError {
    fn from(err: jsonrpc_client_core::Error) -> MenuError {
        MenuError::OledClient(err)
    }
}

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

#[derive(Debug)]
pub enum StatsError {
    StatsHttp(jsonrpc_client_http::Error),
    StatsClient(jsonrpc_client_core::Error),
    StatsSerde(serde_json::error::Error),
}

impl From<jsonrpc_client_http::Error> for StatsError {
    fn from(err: jsonrpc_client_http::Error) -> StatsError {
        StatsError::StatsHttp(err)
    }
}

impl From<jsonrpc_client_core::Error> for StatsError {
    fn from(err: jsonrpc_client_core::Error) -> StatsError {
        StatsError::StatsClient(err)
    }
}

impl From<serde_json::error::Error> for StatsError {
    fn from(err: serde_json::error::Error) -> StatsError {
        StatsError::StatsSerde(err)
    }
}
