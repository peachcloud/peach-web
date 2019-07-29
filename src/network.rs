extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::MenuError;

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_ip` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_get_ip(iface: String) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_ip(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_rssi` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_get_rssi(iface: String) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_rssi(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_ssid` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_get_ssid(iface: String) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_ssid(iface).call()?;

    Ok(response)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// Creates a JSON-RPC request to get the IP address for the given interface.
    pub fn get_ip(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the average signal strength for the given interface.
    pub fn get_rssi(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn get_ssid(&mut self, iface: String) -> RpcRequest<String>;
});
