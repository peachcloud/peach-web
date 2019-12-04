extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

// -> create this error.rs
use crate::error::NetworkError;
use crate::structs::{Networks, Traffic};

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_ap` method.
///
pub fn network_activate_ap() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_ap().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `activate_client` method.
///
pub fn network_activate_client() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.activate_client().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `add_wifi` method.
///
/// # Arguments
///
/// * `ssid` - A String containing the SSID of an access point.
///
pub fn network_add_wifi(ssid: String, pass: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.add_wifi(ssid, pass).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_ip` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_get_ip(iface: String) -> std::result::Result<String, NetworkError> {
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
pub fn network_get_rssi(iface: String) -> std::result::Result<String, NetworkError> {
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
pub fn network_get_ssid(iface: String) -> std::result::Result<String, NetworkError> {
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

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_state` method.
///
pub fn network_get_state(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_state(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_stats` method.
///
pub fn network_get_traffic(iface: String) -> std::result::Result<Traffic, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_traffic(iface).call()?;
    let t: Traffic = serde_json::from_str(&response).unwrap();

    Ok(t)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `reconnect_wifi` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_reconnect_wifi(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.reconnect_wifi(iface).call()?;

    Ok(response)
}

pub fn network_scan_networks(iface: String) -> std::result::Result<Networks, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.scan_networks(iface).call()?;
    //let n: Networks = serde_json::from_str(&response).unwrap();
    let n: Networks = serde_json::from_str(&response)?;

    Ok(n)
    //Ok(response)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// Creates a JSON-RPC request to activate the access point.
    pub fn activate_ap(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to activate the wireless client (wlan0).
    pub fn activate_client(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to save credentials for an access point.
    pub fn add_wifi(&mut self, ssid: String, pass: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the IP address for the given interface.
    pub fn get_ip(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the average signal strength for the given interface.
    pub fn get_rssi(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn get_ssid(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the state for the given interface.
    pub fn get_state(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to reconnect WiFi for the given interface.
    pub fn reconnect_wifi(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the network traffic for the given interface.
    pub fn get_traffic(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to list all networks in range of the given interface.
    pub fn scan_networks(&mut self, iface: String) -> RpcRequest<String>;
});
