extern crate jsonrpc_client_http;

use std::collections::HashMap;
use std::env;

use jsonrpc_client_http::HttpTransport;

// -> create this error.rs
use crate::error::NetworkError;
use crate::structs::{NetworkListContext, Networks, Traffic};

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
pub fn network_add(ssid: String, pass: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.add(ssid, pass).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `available_networks` method, which returns a list of in-range access points.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_available_networks(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.available_networks(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `connect` method, which disables other network connections and enables the
/// connection for the chosen network, identified by ID and interface.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_connect(id: String, iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.connect(id, iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `delete` method, which removes the credentials of the given network
/// from the wpa_configuration file.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_delete(id: &str, iface: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.delete(id, iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
/// * `ssid` - A String containing the SSID of a network.
///
pub fn network_id(iface: &str, ssid: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.id(iface, ssid).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ip` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_ip(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ip(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `modify` method, which replaces the old network access point password
/// with a new one. The access point is identified by ID on a given interface.
///
/// # Arguments
///
/// * `id` - A String containing a network identifier.
/// * `iface` - A String containing the network interface identifier.
/// * `pass` - A String containing the new password.
///
pub fn network_modify(
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
    let response = client.modify(id, iface, pass).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `reconfigure` method.
///
pub fn network_reconfigure() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.reconfigure().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `reconnect` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_reconnect(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.reconnect(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `rssi` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_rssi(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.rssi(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `rssi_percent` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_rssi_percent(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.rssi_percent(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `save` method.
///
pub fn network_save() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.save().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `saved_networks` method, which returns a list of networks saved in
/// `wpa_supplicant.conf`.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_saved_networks() -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);
    let response = client.saved_networks().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `ssid` method.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_ssid(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.ssid(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `state` method.
///
pub fn network_state(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.state(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `status` method.
///
pub fn network_status(iface: String) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.status(iface).call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `traffic` method.
///
pub fn network_traffic(iface: String) -> std::result::Result<Traffic, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    let response = client.traffic(iface).call()?;
    let t: Traffic = serde_json::from_str(&response).unwrap();

    Ok(t)
}

// BUNDLED METHODS
//  - perform multiple RPC calls with one transport

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `delete` and `save` methods.
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

    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    info!("Performing delete call to peach-network microservice.");
    client.delete(&id, &iface).call()?;
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `modify` and `save` methods.
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

    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    info!("Performing modify call to peach-network microservice.");
    client.modify(&id, &iface, &pass).call()?;
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// This function retries the data required to build the NetworkListContext
/// object. Creates a JSON-RPC client with http transport and calls the
/// `peach-network` `saved_networks`, `available_networks` and `ssid` methods.
///
/// # Arguments
///
/// * `iface` - A String containing the network interface identifier.
///
pub fn network_list_context(iface: &str) -> std::result::Result<NetworkListContext, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    // list of networks saved in the wpa_supplicant.conf
    let wlan_list = match client.saved_networks().call() {
        Ok(ssids) => {
            let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                .expect("Failed to deserialize scan_list response");
            networks
        }
        Err(_) => Vec::new(),
    };

    // list of networks currently in range (online & accessible)
    let wlan_scan = match client.available_networks(iface.to_string()).call() {
        Ok(networks) => {
            let scan: Vec<Networks> = serde_json::from_str(networks.as_str())
                .expect("Failed to deserialize scan_networks response");
            scan
        }
        Err(_) => Vec::new(),
    };
    let wlan_ssid = match client.ssid(iface.to_string()).call() {
        Ok(ssid) => ssid,
        Err(_) => "Not connected".to_string(),
    };

    // create a hashmap to combine wlan_list & wlan_scan without repetition
    let mut wlan_networks = HashMap::new();
    for ap in wlan_scan {
        wlan_networks.insert(ap.ssid, "Available".to_string());
    }
    for network in wlan_list {
        // insert ssid (with state) only if it doesn't already exist
        wlan_networks
            .entry(network.ssid)
            .or_insert_with(|| "Not in range".to_string());
    }

    let context = NetworkListContext {
        wlan_networks,
        wlan_ssid,
        flash_name: None,
        flash_msg: None,
        back: None,
    };

    Ok(context)
}

jsonrpc_client!(pub struct PeachNetworkClient {
    /// JSON-RPC request to activate the access point.
    pub fn activate_ap(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to activate the wireless client (wlan0).
    pub fn activate_client(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to add credentials for an access point.
    pub fn add(&mut self, ssid: String, pass: String) -> RpcRequest<String>;

    /// JSON-RPC request to list all networks in range of the given interface.
    pub fn available_networks(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to connect the network for the given interface and ID.
    pub fn connect(&mut self, id: String, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to delete the credentials for the given network from the wpa_supplicant config.
    pub fn delete(&mut self, id: &str, iface: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the ID for the given interface and SSID.
    pub fn id(&mut self, iface: &str, ssid: &str) -> RpcRequest<String>;

    /// JSON-RPC request to get the IP address for the given interface.
    pub fn ip(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to set a new network password for the given interface and ID.
    pub fn modify(&mut self, id: &str, iface: &str, pass: &str) -> RpcRequest<String>;

    /// JSON-RPC request to reread the wpa_supplicant config for the given interface.
    pub fn reconfigure(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to reconnect WiFi for the given interface.
    pub fn reconnect(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to get the average signal strength (dBm) for the given interface.
    pub fn rssi(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to get the average signal quality (%) for the given interface.
    pub fn rssi_percent(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to save network configuration updates to file.
    pub fn save(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to list all networks saved in `wpa_supplicant.conf`.
    pub fn saved_networks(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get the SSID of the currently-connected network for the given interface.
    pub fn ssid(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to get the state for the given interface.
    pub fn state(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to get the status of the given interface.
    pub fn status(&mut self, iface: String) -> RpcRequest<String>;

    /// JSON-RPC request to get the network traffic for the given interface.
    pub fn traffic(&mut self, iface: String) -> RpcRequest<String>;
});
