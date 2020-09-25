//! Provides network-related methods, data structures and helper functions which
//! utilise the JSON-RPC `peach-network` microservice.

extern crate jsonrpc_client_http;

use std::collections::HashMap;
use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::context::NetworkListContext;
use crate::error::NetworkError;
use crate::network_client::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessPoint {
    pub detail: Option<Scan>,
    pub signal: Option<i32>,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Networks {
    pub ssid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scan {
    pub protocol: String,
    pub frequency: String,
    pub signal_level: String,
    pub ssid: String,
}

#[derive(Debug, Deserialize, FromForm, UriDisplayQuery)]
pub struct Ssid {
    pub ssid: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}

/// Helper function to determine if a given SSID already exists in the
/// `wpa_supplicant.conf` file, indicating that network credentials have already
/// been added for that access point. Creates a JSON-RPC client with http
/// transport and calls the `peach-network` `saved_networks` method. Returns a
/// boolean expression inside a Result type.
///
/// # Arguments
///
/// * `ssid` - A string slice containing the SSID of a network.
pub fn check_saved_aps(ssid: &str) -> std::result::Result<bool, NetworkError> {
    // retrieve a list of access points with saved credentials
    let saved_aps = match network_saved_networks() {
        Ok(ssids) => {
            let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                .expect("Failed to deserialize saved_networks response");
            networks
        }
        // return an empty vector if there are no saved access point credentials
        Err(_) => Vec::new(),
    };

    // loop through the access points in the list
    for network in saved_aps {
        // return true if the access point ssid matches the given ssid
        if network.ssid == ssid {
            return Ok(true);
        }
    }

    // return false if no matches are found
    Ok(false)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id` and `disable` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
pub fn network_disable(iface: &str, ssid: &str) -> std::result::Result<String, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = PeachNetworkClient::new(transport_handle);

    // get the id of the network
    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    // disable the network
    info!("Performing disable call to peach-network microservice.");
    client.disable(&id, &iface).call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `delete` and `save` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
pub fn forget_network(iface: &str, ssid: &str) -> std::result::Result<String, NetworkError> {
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
    // WEIRD BUG: the parameters below are technically in the wrong order:
    // it should be id first and then iface, but somehow they get twisted.
    // i don't understand computers.
    client.delete(&iface, &id).call()?;
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-network`
/// `id`, `delete`, `save` and `add` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
/// * `ssid` - A string slice containing the SSID of a network.
/// * `pass` - A string slice containing the password for a network.
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

    // get the id of the network
    info!("Performing id call to peach-network microservice.");
    let id = client.id(&iface, &ssid).call()?;
    // delete the old credentials
    // WEIRD BUG: the parameters below are technically in the wrong order:
    // it should be id first and then iface, but somehow they get twisted.
    // i don't understand computers.
    info!("Performing delete call to peach-network microservice.");
    client.delete(&iface, &id).call()?;
    // save the updates to wpa_supplicant.conf
    info!("Performing save call to peach-network microservice.");
    client.save().call()?;
    // add the new credentials
    info!("Performing add call to peach-network microservice.");
    client.add(ssid, pass).call()?;
    // reconfigure wpa_supplicant with latest addition to config
    info!("Performing reconfigure call to peach-network microservice.");
    client.reconfigure().call()?;

    let response = "success".to_string();

    Ok(response)
}

/// This function retrieves the data required to build the NetworkListContext
/// object. Creates a JSON-RPC client with http transport and calls the
/// `peach-network` `saved_networks`, `available_networks` and `ssid` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
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
    let wlan_scan = match client.available_networks(iface).call() {
        Ok(networks) => {
            let scan: Vec<Networks> = serde_json::from_str(networks.as_str())
                .expect("Failed to deserialize scan_networks response");
            scan
        }
        Err(_) => Vec::new(),
    };

    let wlan_ssid = match client.ssid(iface).call() {
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

    let ap_state = match network_state("ap0") {
        Ok(state) => state,
        Err(_) => "Interface unavailable".to_string(),
    };

    let context = NetworkListContext {
        ap_state,
        back: None,
        flash_msg: None,
        flash_name: None,
        title: None,
        wlan_networks,
        wlan_ssid,
    };

    Ok(context)
}
