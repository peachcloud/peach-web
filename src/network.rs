extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

// -> create this error.rs
use crate::error::NetworkError;
use crate::structs::Traffic;

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
/// `get_id` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
/// * `ssid` - A String containing the SSID of a network.
///
pub fn network_get_id(iface: &str, ssid: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_id(iface, ssid).call()?;

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
/// `list_networks` method, which returns a list of networks saved in
/// `wpa_supplicant.conf`.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_list_networks() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);
    let response = client.list_networks().call()?;

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
/// `get_status` method.
///
pub fn network_get_status(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.get_status(iface).call()?;

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
/// `new_password` method, which replaces the old network access point password
/// with a new one. The access point is identified by ID on a given interface.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
/// * `pass` - A String containing the new password.
///
pub fn network_new_password(
    id: &str,
    iface: &str,
    pass: &str,
) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);
    let response = client.new_password(id, iface, pass).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `reconfigure_wifi` method.
///
pub fn network_reconfigure_wifi() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.reconfigure_wifi().call()?;

    Ok(response)
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

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `remove_wifi` method, which removes the credentials of the given network
/// from the wpa_configuration file.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_remove_wifi(id: &str, iface: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.remove_wifi(id, iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `save_config` method.
///
pub fn network_save_config() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.save_config().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `scan_networks` method, which returns a list of in-range access points.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_scan_networks(iface: String) -> std::result::Result<String, NetworkError> {
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

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `select_network` method, which disables other network connections and
/// enables the connection for the chosen network, identified by ID and
/// interface.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_select_network(
    id: String,
    iface: String,
) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.select_network(id, iface).call()?;

    Ok(response)
}

// BUNDLED METHODS
//  - perform multiple RPC calls with one transport

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_id`, `remove_wifi` and `save_config` methods.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
/// * `ssid` - A String containing the SSID of a network.
///
pub fn forget_network(iface: String, ssid: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    info!("Performing get_id call to peach-network microservice.");
    let id = client.get_id(&iface, &ssid).call()?;
    info!("Performing remove_wifi call to peach-network microservice.");
    client.remove_wifi(&id, &iface).call()?;
    info!("Performing save_config call to peach-network microservice.");
    client.save_config().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `get_id`, `remove_wifi` and `save_config` methods.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
/// * `pass` - A string slice containing the password for a network.
///
pub fn update_password(
    iface: &str,
    ssid: &str,
    pass: &str,
) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    info!("Performing get_id call to peach-network microservice.");
    let id = client.get_id(&iface, &ssid).call()?;
    info!("Performing new_password call to peach-network microservice.");
    client.new_password(&id, &iface, &pass).call()?;
    info!("Performing save_config call to peach-network microservice.");
    client.save_config().call()?;

    let response = "success".to_string();

    Ok(response)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// Creates a JSON-RPC request to activate the access point.
    pub fn activate_ap(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to activate the wireless client (wlan0).
    pub fn activate_client(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to save credentials for an access point.
    pub fn add_wifi(&mut self, ssid: String, pass: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the ID for the given interface and SSID.
    pub fn get_id(&mut self, iface: &str, ssid: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the IP address for the given interface.
    pub fn get_ip(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the average signal strength for the given interface.
    pub fn get_rssi(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn get_ssid(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the state for the given interface.
    pub fn get_state(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the status of the given interface.
    pub fn get_status(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to get the network traffic for the given interface.
    pub fn get_traffic(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to list all networks saved in `wpa_supplicant.conf`.
    pub fn list_networks(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to set a new network password for the given interface and ID.
    pub fn new_password(&mut self, id: &str, iface: &str, pass: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to reread the wpa_supplicant config for the given interface.
    pub fn reconfigure_wifi(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to reconnect WiFi for the given interface.
    pub fn reconnect_wifi(&mut self, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to remove the credentials for the given network from the wpa_supplicant config.
    pub fn remove_wifi(&mut self, id: &str, iface: &str) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to save network configuration updates to file.
    pub fn save_config(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to select the network for the given interface and ID.
    pub fn select_network(&mut self, id: String, iface: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to list all networks in range of the given interface.
    pub fn scan_networks(&mut self, iface: String) -> RpcRequest<String>;
});
