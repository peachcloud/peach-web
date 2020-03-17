use std::collections::HashMap;

use rocket_contrib::json::JsonValue;

use crate::network::*;

#[derive(Debug, Serialize)]
pub struct FlashContext {
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

// used in /network/wifi/add?<ssid>
#[derive(Debug, Serialize)]
pub struct NetworkAddContext {
    pub back: Option<String>,
    pub selected: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NetworkDetailContext {
    pub wlan_ip: String,
    //pub wlan_list: Vec<Networks>,
    pub wlan_networks: HashMap<String, Scan>,
    pub wlan_rssi: Option<String>,
    //pub wlan_scan: Vec<Scan>,
    pub wlan_ssid: String,
    pub wlan_state: String,
    pub wlan_status: String,
    pub wlan_traffic: Option<Traffic>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    // allows for passing in the ssid of a chosen access point
    // this is used in the network_detail template
    pub selected: Option<String>,
    pub back: Option<String>,
}

impl NetworkDetailContext {
    pub fn build() -> NetworkDetailContext {
        let wlan_ip = match network_get_ip("wlan0".to_string()) {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        // list of networks saved in wpa_supplicant.conf
        let wlan_list = match network_list_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };
        let wlan_rssi = match network_get_rssi("wlan0".to_string()) {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        // list of networks currently in range (online & accessible)
        let wlan_scan = match network_scan_networks("wlan0".to_string()) {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                scan
            }
            Err(_) => Vec::new(),
        };
        let wlan_ssid = match network_get_ssid("wlan0".to_string()) {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_get_state("wlan0".to_string()) {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_get_status("wlan0".to_string()) {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_get_traffic("wlan0".to_string()) {
            Ok(traffic) => {
                let mut t = traffic;
                // modify traffic values & assign measurement unit
                // based on received and transmitted values
                // if received > 999 MB, convert it to GB
                if t.received > 1_047_527_424 {
                    t.received /= 1_073_741_824;
                    t.rx_unit = Some("GB".to_string());
                } else if t.received > 0 {
                    // otherwise, convert it to MB
                    t.received = (t.received / 1024) / 1024;
                    t.rx_unit = Some("MB".to_string());
                } else {
                    t.received = 0;
                    t.rx_unit = Some("MB".to_string());
                }

                if t.transmitted > 1_047_527_424 {
                    t.transmitted /= 1_073_741_824;
                    t.tx_unit = Some("GB".to_string());
                } else if t.transmitted > 0 {
                    t.transmitted = (t.transmitted / 1024) / 1024;
                    t.tx_unit = Some("MB".to_string());
                } else {
                    t.transmitted = 0;
                    t.tx_unit = Some("MB".to_string());
                }
                Some(t)
            }
            Err(_) => None,
        };
        // create a hashmap to combine wlan_list & wlan_scan without repetition
        let mut wlan_networks = HashMap::new();
        for mut ap in wlan_scan {
            let ssid = ap.ssid.clone();
            ap.state = "Available".to_string();
            wlan_networks.insert(ssid, ap);
        }
        for network in wlan_list {
            // avoid repetition by checking that ssid is not already in list
            if !wlan_networks.contains_key(&network.ssid) {
                let ssid = network.ssid.clone();
                let net = Scan {
                    protocol: None,
                    frequency: None,
                    signal_level: None,
                    ssid: network.ssid,
                    state: "Not in range".to_string(),
                };
                wlan_networks.insert(ssid, net);
            }
        }
        
        NetworkDetailContext {
            wlan_ip,
            //wlan_list,
            wlan_networks,
            wlan_rssi,
            //wlan_scan,
            wlan_ssid,
            wlan_state,
            wlan_status,
            wlan_traffic,
            flash_name: None,
            flash_msg: None,
            selected: None,
            back: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NetworkContext {
    pub ap_ip: String,
    pub ap_ssid: String,
    pub ap_state: String,
    pub ap_traffic: Option<Traffic>,
    pub wlan_ip: String,
    pub wlan_rssi: Option<String>,
    pub wlan_scan: Option<Vec<Scan>>,
    pub wlan_ssid: String,
    pub wlan_state: String,
    pub wlan_status: String,
    pub wlan_traffic: Option<Traffic>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    // allows for passing in the ssid of a chosen access point
    // this is used in the network_detail template
    pub selected: Option<String>,
    // url for back-arrow link
    pub back: Option<String>,
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
        let ap_traffic = match network_get_traffic("ap0".to_string()) {
            Ok(traffic) => {
                let mut t = traffic;
                // modify traffic values & assign measurement unit
                // based on received and transmitted values
                // if received > 999 MB, convert it to GB
                if t.received > 1_047_527_424 {
                    t.received /= 1_073_741_824;
                    t.rx_unit = Some("GB".to_string());
                } else if t.received > 0 {
                    // otherwise, convert it to MB
                    t.received = (t.received / 1024) / 1024;
                    t.rx_unit = Some("MB".to_string());
                } else {
                    t.received = 0;
                    t.rx_unit = Some("MB".to_string());
                }

                if t.transmitted > 1_047_527_424 {
                    t.transmitted /= 1_073_741_824;
                    t.tx_unit = Some("GB".to_string());
                } else if t.transmitted > 0 {
                    t.transmitted = (t.transmitted / 1024) / 1024;
                    t.tx_unit = Some("MB".to_string());
                } else {
                    t.transmitted = 0;
                    t.tx_unit = Some("MB".to_string());
                }
                Some(t)
            }
            Err(_) => None,
        };
        let wlan_ip = match network_get_ip("wlan0".to_string()) {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let wlan_rssi = match network_get_rssi("wlan0".to_string()) {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        let wlan_scan = match network_scan_networks("wlan0".to_string()) {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                Some(scan)
            }
            Err(_) => None,
        };
        let wlan_ssid = match network_get_ssid("wlan0".to_string()) {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_get_state("wlan0".to_string()) {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_get_status("wlan0".to_string()) {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_get_traffic("wlan0".to_string()) {
            Ok(traffic) => {
                let mut t = traffic;
                // modify traffic values & assign measurement unit
                // based on received and transmitted values
                // if received > 999 MB, convert it to GB
                if t.received > 1_047_527_424 {
                    t.received /= 1_073_741_824;
                    t.rx_unit = Some("GB".to_string());
                } else if t.received > 0 {
                    // otherwise, convert it to MB
                    t.received = (t.received / 1024) / 1024;
                    t.rx_unit = Some("MB".to_string());
                } else {
                    t.received = 0;
                    t.rx_unit = Some("MB".to_string());
                }

                if t.transmitted > 1_047_527_424 {
                    t.transmitted /= 1_073_741_824;
                    t.tx_unit = Some("GB".to_string());
                } else if t.transmitted > 0 {
                    t.transmitted = (t.transmitted / 1024) / 1024;
                    t.tx_unit = Some("MB".to_string());
                } else {
                    t.transmitted = 0;
                    t.tx_unit = Some("MB".to_string());
                }
                Some(t)
            }
            Err(_) => None,
        };

        NetworkContext {
            ap_ip,
            ap_ssid,
            ap_state,
            ap_traffic,
            wlan_ip,
            wlan_rssi,
            wlan_scan,
            wlan_ssid,
            wlan_state,
            wlan_status,
            wlan_traffic,
            flash_name: None,
            flash_msg: None,
            selected: None,
            back: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NetworkListContext {
    pub wlan_ssid: String,
    pub wlan_networks: HashMap<String, String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub back: Option<String>,
}

impl NetworkListContext {
    pub fn build() -> NetworkListContext {
        // list of networks saved in the wpa_supplicant.conf
        let wlan_list = match network_list_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };
        // list of networks currently in range (online & accessible)
        let wlan_scan = match network_scan_networks("wlan0".to_string()) {
            Ok(networks) => {
                let scan: Vec<Networks> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                scan
            }
            Err(_) => Vec::new(),
        };
        let wlan_ssid = match network_get_ssid("wlan0".to_string()) {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        // create a hashmap to combine wlan_list & wlan_scan without repetition
        let mut wlan_networks = HashMap::new();
        for ap in wlan_scan {
            wlan_networks.insert(ap.ssid, "Available".to_string());
        }
        for network in wlan_list {
            // avoid repetition by checking that ssid is not already in list
            if !wlan_networks.contains_key(&network.ssid) {
                wlan_networks.insert(network.ssid, "Not in range".to_string());
            }
        }

        NetworkListContext {
            wlan_networks,
            wlan_ssid,
            flash_name: None,
            flash_msg: None,
            back: None,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Networks {
    pub ssid: String,
}

#[derive(Debug, FromForm)]
pub struct Ssid {
    pub ssid: String,
}

/*
#[derive(Debug, Deserialize, Serialize)]
pub struct Scan {
    pub protocol: String,
    pub frequency: String,
    pub signal_level: String,
    pub ssid: String,
}
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Scan {
    pub protocol: Option<String>,
    pub frequency: Option<String>,
    pub signal_level: Option<String>,
    pub ssid: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Traffic {
    pub received: u64,
    pub transmitted: u64,
    pub rx_unit: Option<String>,
    pub tx_unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Uptime {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(Debug, FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}
