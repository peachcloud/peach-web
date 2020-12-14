//! Provides network-related methods, data structures and helper functions which
//! utilise the JSON-RPC `peach-network` microservice.

use std::collections::HashMap;
use std::env;

use jsonrpc_client_http::HttpTransport;
use log::{debug, info};
use rocket::request::FromForm;
use rocket::UriDisplayQuery;
use serde::Deserialize;

use peach_lib::network_client;
use peach_lib::network_client::Networks;

use crate::context::NetworkListContext;
use crate::error::NetworkError;

#[derive(Debug, Deserialize, FromForm, UriDisplayQuery)]
pub struct Ssid {
    pub ssid: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}

/// This function retrieves the data required to build the NetworkListContext
/// object. Creates a JSON-RPC client with http transport and calls the
/// `peach-network` `saved_networks`, `available_networks` and `ssid` methods.
///
/// # Arguments
///
/// * `iface` - A string slice containing the network interface identifier.
pub fn list_context(iface: &str) -> std::result::Result<NetworkListContext, NetworkError> {
    debug!("Creating HTTP transport for network client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr =
        env::var("PEACH_NETWORK_SERVER").unwrap_or_else(|_| "127.0.0.1:5110".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_network service.");
    let mut client = network_client::PeachNetworkClient::new(transport_handle);

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

    let ap_state = match network_client::state("ap0") {
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
