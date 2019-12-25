//use serde::{Deserialize, Serialize};
use rocket_contrib::json::JsonValue;

use crate::network::*;

#[derive(Debug, Serialize)]
pub struct NetworkContext {
    pub ap_ip: String,
    pub ap_ssid: String,
    pub ap_state: String,
    pub wlan_ip: String,
    pub wlan_scan: String,
    pub wlan_ssid: String,
    pub wlan_state: String,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl NetworkContext {
    pub fn build() -> NetworkContext {
        let ap_ip = match network_get_ip("ap0".to_string()) {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let ap_ssid = match network_get_ssid("ap0".to_string()) {
            Ok(ssid) => ssid,
            Err(_) => "Not currently activated".to_string(),
        };
        let ap_state = match network_get_state("ap0".to_string()) {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_ip = match network_get_ip("wlan0".to_string()) {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let wlan_scan = match network_scan_networks("wlan0".to_string()) {
            Ok(networks) => networks.list,
            Err(_) => "No WiFi networks found".to_string(),
        };
        let wlan_ssid = match network_get_ssid("wlan0".to_string()) {
            Ok(ssid) => ssid,
            Err(_) => "Not currently connected".to_string(),
        };
        let wlan_state = match network_get_state("wlan0".to_string()) {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };

        NetworkContext {
            ap_ip,
            ap_ssid,
            ap_state,
            wlan_ip,
            wlan_scan,
            wlan_ssid,
            wlan_state,
            flash_name: None,
            flash_msg: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CpuStat {
    pub user: u64,
    pub system: u64,
    pub idle: u64,
    pub nice: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CpuStatPercentages {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub nice: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiskUsage {
    pub filesystem: Option<String>,
    pub one_k_blocks: u64,
    pub one_k_blocks_used: u64,
    pub one_k_blocks_free: u64,
    pub used_percentage: u32,
    pub mountpoint: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoadAverage {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MemStat {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}

#[derive(Debug, Deserialize)]
pub struct Networks {
    pub list: String,
}

#[derive(Debug, Deserialize)]
pub struct Traffic {
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Debug, Deserialize)]
pub struct Uptime {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}
