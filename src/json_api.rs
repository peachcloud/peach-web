//! JSON API routes for PeachCloud.
//!
//! This module contains handlers which allow retrieval and modification of
//! device state via JSON.
//!
//! API ROUTES
//!
//! | Method | URL                              | Description                   |
//! | ------ | -------------------------------- | ----------------------------- |
//! | POST   | /api/v1/device/reboot            | Reboot device                 |
//! | POST   | /api/v1/device/shutdown          | Shutdown device               |
//! | POST   | /api/v1/network/activate_ap      |                               |
//! | POST   | /api/v1/network/activate_client  |                               |
//! | GET    | /api/v1/network/ip               |                               |
//! | GET    | /api/v1/network/rssi             |                               |
//! | GET    | /api/v1/network/ssid             |                               |
//! | GET    | /api/v1/network/state            |                               |
//! | GET    | /api/v1/network/status           |                               |
//! | GET    | /api/v1/network/wifi             | Retrieve available networks   |
//! | POST   | /api/v1/network/wifi             | Add WiFi AP credentials       |
//! | POST   | /api/v1/network/wifi/connect     | Connect to WiFi access point  |
//! | POST   | /api/v1/network/wifi/disconnect  | Disconnect WiFi access point  |
//! | POST   | /api/v1/network/wifi/forget      | Forget / remove network       |
//! | POST   | /api/v1/network/wifi/modify      | Modify network password       |
//! | POST   | /api/v1/network/wifi/usage       | Update alert thresholds       |
//! | POST   | /api/v1/network/wifi/usage/reset | Reset stored data usage total |
//! | GET    | /api/v1/ping                     |                               |
//! | GET    | /api/v1/ping/network             | Ping `peach-network`          |
//! | GET    | /api/v1/ping/oled                | Ping `peach-oled`             |
//! | GET    | /api/v1/ping/stats               | Ping `peach-stats`            |

use log::{debug, warn};
use rocket::{get, post};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::Serialize;

use peach_lib::network_client;
use peach_lib::oled_client;
use peach_lib::stats_client;
use peach_lib::stats_client::Traffic;

use crate::device;
use crate::monitor;
use crate::monitor::Threshold;
use crate::network;
use crate::network::{Ssid, WiFi};

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

// reboot the device
#[post("/api/v1/device/reboot")]
pub fn reboot_device() -> Json<JsonResponse> {
    match device::reboot() {
        Ok(_) => {
            debug!("Going down for reboot...");
            let status = "success".to_string();
            let msg = "Going down for reboot.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("Reboot failed");
            let status = "error".to_string();
            let msg = "Failed to reboot the device.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// shutdown the device
#[post("/api/v1/device/shutdown")]
pub fn shutdown_device() -> Json<JsonResponse> {
    match device::shutdown() {
        Ok(_) => {
            debug!("Going down for shutdown...");
            let status = "success".to_string();
            let msg = "Going down for shutdown.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("Shutdown failed");
            let status = "error".to_string();
            let msg = "Failed to shutdown the device.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/activate_ap")]
pub fn activate_ap() -> Json<JsonResponse> {
    // activate the wireless access point
    debug!("Activating WiFi access point.");
    match network_client::activate_ap() {
        Ok(_) => {
            let status = "success".to_string();
            Json(build_json_response(status, None, None))
        }
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to activate WiFi access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/activate_client")]
pub fn activate_client() -> Json<JsonResponse> {
    // activate the wireless client
    debug!("Activating WiFi client mode.");
    match network_client::activate_client() {
        Ok(_) => {
            let status = "success".to_string();
            Json(build_json_response(status, None, None))
        }
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to activate WiFi client mode.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/ip")]
pub fn return_ip() -> Json<JsonResponse> {
    // retrieve ip for wlan0 or set to x.x.x.x if not found
    let wlan_ip = match network_client::ip("wlan0") {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };
    // retrieve ip for ap0 or set to x.x.x.x if not found
    let ap_ip = match network_client::ip("ap0") {
        Ok(ip) => ip,
        Err(_) => "x.x.x.x".to_string(),
    };
    let data = json!({
        "wlan0": wlan_ip,
        "ap0": ap_ip
    });
    let status = "success".to_string();
    Json(build_json_response(status, Some(data), None))
}

#[get("/api/v1/network/rssi")]
pub fn return_rssi() -> Json<JsonResponse> {
    // retrieve rssi for connected network
    match network_client::rssi("wlan0") {
        Ok(rssi) => {
            let status = "success".to_string();
            let data = json!(rssi);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/ssid")]
pub fn return_ssid() -> Json<JsonResponse> {
    // retrieve ssid for connected network
    match network_client::ssid("wlan0") {
        Ok(network) => {
            let status = "success".to_string();
            let data = json!(network);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/state")]
pub fn return_state() -> Json<JsonResponse> {
    // retrieve state of wlan0 or set to x.x.x.x if not found
    let wlan_state = match network_client::state("wlan0") {
        Ok(state) => state,
        Err(_) => "unavailable".to_string(),
    };
    // retrieve state for ap0 or set to x.x.x.x if not found
    let ap_state = match network_client::state("ap0") {
        Ok(state) => state,
        Err(_) => "unavailable".to_string(),
    };
    let data = json!({
        "wlan0": wlan_state,
        "ap0": ap_state
    });
    let status = "success".to_string();
    Json(build_json_response(status, Some(data), None))
}

#[get("/api/v1/network/status")]
pub fn return_status() -> Json<JsonResponse> {
    // retrieve status info for wlan0 interface
    match network_client::status("wlan0") {
        Ok(network) => {
            let status = "success".to_string();
            let data = json!(network);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Not currently connected to an access point.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[get("/api/v1/network/wifi")]
pub fn scan_networks() -> Json<JsonResponse> {
    // retrieve scan results for access-points within range of wlan0
    match network_client::available_networks("wlan0") {
        Ok(networks) => {
            let status = "success".to_string();
            let data = json!(networks);
            Json(build_json_response(status, Some(data), None))
        }
        Err(_) => {
            let status = "success".to_string();
            let msg = "Unable to scan for networks. Interface may be deactivated.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi", data = "<wifi>")]
pub fn add_wifi(wifi: Json<WiFi>) -> Json<JsonResponse> {
    // generate and write wifi config to wpa_supplicant
    match network_client::add(&wifi.ssid, &wifi.pass) {
        Ok(_) => {
            debug!("Added WiFi credentials.");
            // force reread of wpa_supplicant.conf file with new credentials
            match network_client::reconfigure() {
                Ok(_) => debug!("Successfully reconfigured wpa_supplicant."),
                Err(_) => warn!("Failed to reconfigure wpa_supplicant."),
            }
            // json response for successful update
            let status = "success".to_string();
            let msg = "WiFi credentials added.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            // json response for failed update
            let status = "error".to_string();
            let msg = "Failed to add WiFi credentials.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/connect", data = "<ssid>")]
pub fn connect_ap(ssid: Json<Ssid>) -> Json<JsonResponse> {
    // retrieve the id for the given network ssid
    match network_client::id("wlan0", &ssid.ssid) {
        // attempt connection with the given network
        Ok(id) => match network_client::connect(&id, "wlan0") {
            Ok(_) => {
                let status = "success".to_string();
                let msg = "Connected to chosen network.".to_string();
                Json(build_json_response(status, None, Some(msg)))
            }
            Err(_) => {
                let status = "error".to_string();
                let msg = "Failed to connect to chosen network.".to_string();
                Json(build_json_response(status, None, Some(msg)))
            }
        },
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to retrieve the network ID.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/disconnect", data = "<ssid>")]
pub fn disconnect_ap(ssid: Json<Ssid>) -> Json<JsonResponse> {
    // attempt to disable the current network for wlan0 interface
    match network::disable("wlan0", &ssid.ssid) {
        Ok(_) => {
            let status = "success".to_string();
            let msg = "Disconnected from WiFi network.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            let status = "error".to_string();
            let msg = "Failed to disconnect from WiFi network.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/forget", data = "<network>")]
pub fn forget_ap(network: Json<Ssid>) -> Json<JsonResponse> {
    let ssid = &network.ssid;
    match network::forget("wlan0", &ssid) {
        Ok(_) => {
            debug!("Removed WiFi credentials for chosen network.");
            let status = "success".to_string();
            let msg = "WiFi network credentials removed.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("Failed to remove WiFi credentials.");
            let status = "error".to_string();
            let msg = "Failed to remove WiFi network credentials.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/modify", data = "<wifi>")]
pub fn modify_password(wifi: Json<WiFi>) -> Json<JsonResponse> {
    let ssid = &wifi.ssid;
    let pass = &wifi.pass;
    // we are using a helper function (`update_password`) to delete the old
    // credentials and add the new ones. this is because the wpa_cli method
    // for updating the password does not work.
    match network::update_password("wlan0", ssid, pass) {
        Ok(_) => {
            debug!("WiFi password updated for chosen network.");
            let status = "success".to_string();
            let msg = "WiFi password updated.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("Failed to update WiFi password.");
            let status = "error".to_string();
            let msg = "Failed to update WiFi password.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/usage", data = "<thresholds>")]
pub fn update_wifi_alerts(thresholds: Json<Threshold>) -> Json<JsonResponse> {
    match monitor::update_store(thresholds.into_inner()) {
        Ok(_) => {
            debug!("WiFi data usage thresholds updated.");
            let status = "success".to_string();
            let msg = "Updated alert threshold and flags.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("Failed to update WiFi data usage thresholds.");
            let status = "error".to_string();
            let msg = "Failed to update WiFi data usage thresholds.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

#[post("/api/v1/network/wifi/usage/reset")]
pub fn reset_data_total() -> Json<JsonResponse> {
    match monitor::reset_data() {
        Ok(_) => {
            debug!("Reset network data usage total.");
            let traffic = match network_client::traffic("wlan0") {
                Ok(t) => t,
                Err(_) => Traffic {
                    received: 0,
                    transmitted: 0,
                    rx_unit: None,
                    tx_unit: None,
                },
            };
            // current wifi traffic values as bytes
            let current_traffic = traffic.received + traffic.transmitted;
            let data = json!(current_traffic);
            let status = "success".to_string();
            let msg = "Reset network data usage total.".to_string();
            Json(build_json_response(status, Some(data), Some(msg)))
        }
        Err(_) => {
            warn!("Failed to reset network data usage total.");
            let status = "error".to_string();
            let msg = "Failed to reset network data usage total.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// status route: useful for checking connectivity from web client
#[get("/api/v1/ping")]
pub fn ping_pong() -> Json<JsonResponse> {
    // ping pong
    let status = "success".to_string();
    let msg = "pong!".to_string();
    Json(build_json_response(status, None, Some(msg)))
}

// status route: check availability of `peach-network` microservice
#[get("/api/v1/ping/network")]
pub fn ping_network() -> Json<JsonResponse> {
    match network_client::ping() {
        Ok(_) => {
            debug!("peach-network responded successfully");
            let status = "success".to_string();
            let msg = "peach-network is available.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("peach-network failed to respond");
            let status = "error".to_string();
            let msg = "peach-network is unavailable.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// status route: check availability of `peach-oled` microservice
#[get("/api/v1/ping/oled")]
pub fn ping_oled() -> Json<JsonResponse> {
    match oled_client::ping() {
        Ok(_) => {
            debug!("peach-oled responded successfully");
            let status = "success".to_string();
            let msg = "peach-oled is available.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("peach-oled failed to respond");
            let status = "error".to_string();
            let msg = "peach-oled is unavailable.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// status route: check availability of `peach-stats` microservice
#[get("/api/v1/ping/stats")]
pub fn ping_stats() -> Json<JsonResponse> {
    match stats_client::ping() {
        Ok(_) => {
            debug!("peach-stats responded successfully");
            let status = "success".to_string();
            let msg = "peach-stats is available.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
        Err(_) => {
            warn!("peach-stats failed to respond");
            let status = "error".to_string();
            let msg = "peach-stats is unavailable.".to_string();
            Json(build_json_response(status, None, Some(msg)))
        }
    }
}

// HELPER FUNCTIONS

pub fn build_json_response(
    status: String,
    data: Option<JsonValue>,
    msg: Option<String>,
) -> JsonResponse {
    JsonResponse { status, data, msg }
}
