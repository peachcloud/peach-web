//! Build context objects for inclusion in Tera templates.
//!
//! Each context object is represented by a struct which implements a build
//! method. Context objects provide the means by which application and device
//! state are made available for rendering in the templates.

use std::collections::HashMap;

use crate::monitor::*;
use crate::network::*;
use crate::network_client::*;
use crate::oled_client::oled_ping;
use crate::stats_client::*;

#[derive(Debug, Serialize)]
pub struct HelpContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl HelpContext {
    pub fn build() -> HelpContext {
        HelpContext {
            back: None,
            flash_name: None,
            flash_msg: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl MessageContext {
    pub fn build() -> MessageContext {
        MessageContext {
            back: None,
            flash_name: None,
            flash_msg: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PeerContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl PeerContext {
    pub fn build() -> PeerContext {
        PeerContext {
            back: None,
            flash_name: None,
            flash_msg: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProfileContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl ProfileContext {
    pub fn build() -> ProfileContext {
        ProfileContext {
            back: None,
            flash_name: None,
            flash_msg: None,
        }
    }
}

// used in /network/wifi/alert for traffic alerts
#[derive(Debug, Serialize)]
pub struct NetworkAlertContext {
    pub alert: Alert,
    pub back: Option<String>,
    pub data: Data,       // stored wifi traffic in bytes
    pub data_total: Data, // combined stored and current wifi traffic in bytes
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub threshold: Threshold,
    pub traffic: Traffic, // current wifi traffic in bytes (since boot)
}

impl NetworkAlertContext {
    pub fn build() -> NetworkAlertContext {
        let alert = get_alerts().unwrap();
        // stored wifi data values as bytes
        let data = get_data().unwrap();
        let threshold = get_thresholds().unwrap();
        // current wifi traffic values as bytes
        let traffic = match network_traffic("wlan0") {
            Ok(t) => t,
            Err(_) => Traffic {
                received: 0,
                transmitted: 0,
                rx_unit: None,
                tx_unit: None,
            },
        };

        let rx_total = data.rx + traffic.received;
        let tx_total = data.tx + traffic.transmitted;
        let data_total = Data {
            rx: rx_total,
            tx: tx_total,
        };

        NetworkAlertContext {
            alert,
            back: None,
            data,
            data_total,
            flash_name: None,
            flash_msg: None,
            threshold,
            traffic,
        }
    }
}

// used in /device for system statistics
#[derive(Debug, Serialize)]
pub struct DeviceContext {
    pub back: Option<String>,
    pub cpu_stat_percent: Option<CpuStatPercentages>,
    pub disk_stats: Vec<DiskUsage>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub load_average: Option<LoadAverage>,
    pub mem_stats: Option<MemStat>,
    pub network_ping: String,
    pub oled_ping: String,
    pub stats_ping: String,
    pub uptime: Option<i32>,
}

impl DeviceContext {
    pub fn build() -> DeviceContext {
        // convert result to Option<CpuStatPercentages>, discard any error
        let cpu_stat_percent = cpu_stats_percent().ok();
        let load_average = load_average().ok();
        let mem_stats = mem_stats().ok();
        let network_ping = match network_ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let oled_ping = match oled_ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let stats_ping = match stats_ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let uptime = match uptime() {
            Ok(mins) => mins,
            Err(_) => "Unavailable".to_string(),
        };

        // serialize disk usage data into Vec<DiskUsage>
        let disk_usage_stats = match disk_usage() {
            Ok(disks) => {
                let partitions: Vec<DiskUsage> = serde_json::from_str(disks.as_str())
                    .expect("Failed to deserialize disk_usage response");
                partitions
            }
            Err(_) => Vec::new(),
        };

        let mut disk_stats = Vec::new();
        // select only the partition we're interested in: /dev/mmcblk0p2 ("/")
        for disk in disk_usage_stats {
            if disk.mountpoint == "/" {
                disk_stats.push(disk);
            }
        }

        // parse the uptime string to a signed integer (for math)
        let uptime_parsed = uptime.parse::<i32>().ok();

        DeviceContext {
            back: None,
            cpu_stat_percent,
            disk_stats,
            flash_name: None,
            flash_msg: None,
            load_average,
            mem_stats,
            network_ping,
            oled_ping,
            stats_ping,
            uptime: uptime_parsed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FlashContext {
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

// used in /network/wifi/add?<ssid>
#[derive(Debug, Serialize)]
pub struct NetworkAddContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub selected: Option<String>,
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
        let ap_ip = match network_ip("ap0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let ap_ssid = match network_ssid("ap0") {
            Ok(ssid) => ssid,
            Err(_) => "Not currently activated".to_string(),
        };
        let ap_state = match network_state("ap0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let ap_traffic = match network_traffic("ap0") {
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
        let wlan_ip = match network_ip("wlan0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let wlan_rssi = match network_rssi_percent("wlan0") {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        let wlan_scan = match network_available_networks("wlan0") {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                Some(scan)
            }
            Err(_) => None,
        };
        let wlan_ssid = match network_ssid("wlan0") {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_state("wlan0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_status("wlan0") {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_traffic("wlan0") {
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
pub struct NetworkDetailContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub saved_aps: Vec<Networks>,
    pub selected: Option<String>,
    pub wlan_ip: String,
    pub wlan_networks: HashMap<String, AccessPoint>,
    pub wlan_rssi: Option<String>,
    pub wlan_ssid: String,
    pub wlan_state: String,
    pub wlan_status: String,
    pub wlan_traffic: Option<Traffic>,
}

impl NetworkDetailContext {
    pub fn build() -> NetworkDetailContext {
        let wlan_ip = match network_ip("wlan0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        // list of networks saved in wpa_supplicant.conf
        let wlan_list = match network_saved_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };
        // list of networks saved in wpa_supplicant.conf
        // HACK: we're running the same function twice (wlan_list)
        // see if we can implement clone for Vec<Networks> instead
        let saved_aps = match network_saved_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };
        let wlan_rssi = match network_rssi_percent("wlan0") {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        // list of networks currently in range (online & accessible)
        let wlan_scan = match network_available_networks("wlan0") {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                scan
            }
            Err(_) => Vec::new(),
        };
        let wlan_ssid = match network_ssid("wlan0") {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_state("wlan0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_status("wlan0") {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_traffic("wlan0") {
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
        for ap in wlan_scan {
            let ssid = ap.ssid.clone();
            let rssi = ap.signal_level.clone();
            // parse the string to a signed integer (for math)
            let rssi_parsed = rssi.parse::<i32>().unwrap();
            // perform rssi (dBm) to quality (%) conversion
            let quality_percent = 2 * (rssi_parsed + 100);
            let ap_detail = AccessPoint {
                detail: Some(ap),
                state: "Available".to_string(),
                signal: Some(quality_percent),
            };
            wlan_networks.insert(ssid, ap_detail);
        }
        for network in wlan_list {
            // avoid repetition by checking that ssid is not already in list
            if !wlan_networks.contains_key(&network.ssid) {
                let ssid = network.ssid.clone();
                let net_detail = AccessPoint {
                    detail: None,
                    state: "Not in range".to_string(),
                    signal: None,
                };
                wlan_networks.insert(ssid, net_detail);
            }
        }

        NetworkDetailContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            saved_aps,
            selected: None,
            wlan_ip,
            wlan_networks,
            wlan_rssi,
            wlan_ssid,
            wlan_state,
            wlan_status,
            wlan_traffic,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NetworkListContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub wlan_networks: HashMap<String, String>,
    pub wlan_ssid: String,
}

impl NetworkListContext {
    pub fn build() -> NetworkListContext {
        network_list_context("wlan0").unwrap()
    }
}
