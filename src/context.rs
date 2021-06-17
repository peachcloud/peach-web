//! Build context objects for inclusion in Tera templates.
//!
//! Each context object is represented by a struct which implements a build
//! method. Context objects provide the means by which application and device
//! state are made available for rendering in the templates.

// Context object struct names:
//
// DeviceContext
// ErrorContext
// FlashContext
// HelpContext
// HomeContext
// LoginContext
// MessageContext
// NetworkContext
// NetworkAddContext
// NetworkAlertContext
// NetworkDetailContext
// NetworkListContext
// PeerContext
// ProfileContext
// ShutdownContext

use std::collections::HashMap;

use serde::Serialize;

use peach_lib::config_manager::load_peach_config;
use peach_lib::dyndns_client;
use peach_lib::dyndns_client::is_dns_updater_online;
use peach_lib::network_client;
use peach_lib::network_client::{AccessPoint, Networks, Scan};
use peach_lib::oled_client;
use peach_lib::stats_client;
use peach_lib::sbot_client;
use peach_lib::stats_client::{CpuStatPercentages, DiskUsage, LoadAverage, MemStat, Traffic};

use crate::monitor;
use crate::monitor::{Alert, Data, Threshold};
use crate::utils::get_dyndns_subdomain;

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
    pub dyndns_enabled: bool,
    pub dyndns_is_online: bool,
    pub config_is_valid: bool,
    pub sbot_is_online: bool,
    pub title: Option<String>,
    pub uptime: Option<i32>,
}

impl DeviceContext {
    pub fn build() -> DeviceContext {
        // convert result to Option<CpuStatPercentages>, discard any error
        let cpu_stat_percent = stats_client::cpu_stats_percent().ok();
        let load_average = stats_client::load_average().ok();
        let mem_stats = stats_client::mem_stats().ok();
        let network_ping = match network_client::ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let oled_ping = match oled_client::ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let stats_ping = match stats_client::ping() {
            Ok(_) => "ONLINE".to_string(),
            Err(_) => "OFFLINE".to_string(),
        };
        let uptime = match stats_client::uptime() {
            Ok(mins) => mins,
            Err(_) => "Unavailable".to_string(),
        };

        // serialize disk usage data into Vec<DiskUsage>
        let disk_usage_stats = match stats_client::disk_usage() {
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

        // dyndns_is_online & config_is_valid
        let dyndns_enabled: bool;
        let dyndns_is_online: bool;
        let config_is_valid: bool;
        let load_peach_config_result = load_peach_config();
        match load_peach_config_result {
            Ok(peach_config) => {
                dyndns_enabled = peach_config.dyn_enabled;
                config_is_valid = true;
                if dyndns_enabled {
                    let is_dyndns_online_result = dyndns_client::is_dns_updater_online();
                    match is_dyndns_online_result {
                        Ok(is_online) => {
                            dyndns_is_online = is_online;
                        }
                        Err(_err) => {
                            dyndns_is_online = false;
                        }
                    }
                } else {
                    dyndns_is_online = false;
                }
            }
            Err(_err) => {
                dyndns_enabled = false;
                dyndns_is_online = false;
                config_is_valid = false;
            }
        }

        // test if go-sbot is running
        let sbot_is_online: bool;
        let sbot_is_online_result = sbot_client::is_sbot_online();
        match sbot_is_online_result {
            Ok(val) => {
                sbot_is_online = val;
            }
            Err(_err) => {
                sbot_is_online = false;
            }
        }

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
            dyndns_enabled,
            dyndns_is_online,
            config_is_valid,
            sbot_is_online,
            title: None,
            uptime: uptime_parsed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl ErrorContext {
    pub fn build() -> ErrorContext {
        ErrorContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FlashContext {
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HelpContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl HelpContext {
    pub fn build() -> HelpContext {
        HelpContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct HomeContext {
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl HomeContext {
    pub fn build() -> HomeContext {
        HomeContext {
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl LoginContext {
    pub fn build() -> LoginContext {
        LoginContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MessageContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl MessageContext {
    pub fn build() -> MessageContext {
        MessageContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ConfigureDNSContext {
    pub external_domain: String,
    pub dyndns_subdomain: String,
    pub enable_dyndns: bool,
    pub is_dyndns_online: bool,
    pub back: Option<String>,
    pub title: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
}

impl ConfigureDNSContext {
    pub fn build() -> ConfigureDNSContext {
        let peach_config = load_peach_config().unwrap();
        let dyndns_fulldomain = peach_config.dyn_domain;
        let is_dyndns_online = is_dns_updater_online().unwrap();
        let dyndns_subdomain =
            get_dyndns_subdomain(&dyndns_fulldomain).unwrap_or(dyndns_fulldomain);
        ConfigureDNSContext {
            external_domain: peach_config.external_domain,
            dyndns_subdomain,
            enable_dyndns: peach_config.dyn_enabled,
            is_dyndns_online,
            back: None,
            title: None,
            flash_name: None,
            flash_msg: None,
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
    // page title for header in navbar
    pub title: Option<String>,
    // url for back-arrow link
    pub back: Option<String>,
}

impl NetworkContext {
    pub fn build() -> NetworkContext {
        let ap_ip = match network_client::ip("ap0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let ap_ssid = match network_client::ssid("ap0") {
            Ok(ssid) => ssid,
            Err(_) => "Not currently activated".to_string(),
        };
        let ap_state = match network_client::state("ap0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let ap_traffic = match network_client::traffic("ap0") {
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
        let wlan_ip = match network_client::ip("wlan0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        let wlan_rssi = match network_client::rssi_percent("wlan0") {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        let wlan_scan = match network_client::available_networks("wlan0") {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                Some(scan)
            }
            Err(_) => None,
        };
        let wlan_ssid = match network_client::ssid("wlan0") {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_client::state("wlan0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_client::status("wlan0") {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_client::traffic("wlan0") {
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
            title: None,
            back: None,
        }
    }
}

// used in /network/wifi/add?<ssid>
#[derive(Debug, Serialize)]
pub struct NetworkAddContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub selected: Option<String>,
    pub title: Option<String>,
}

impl NetworkAddContext {
    pub fn build() -> NetworkAddContext {
        NetworkAddContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            selected: None,
            title: None,
        }
    }
}

// used in /network/wifi/alert for traffic alerts
#[derive(Debug, Serialize)]
pub struct NetworkAlertContext {
    pub alert: Alert,
    pub back: Option<String>,
    pub data_total: Data, // combined stored and current wifi traffic in bytes
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub threshold: Threshold,
    pub title: Option<String>,
    pub traffic: Traffic, // current wifi traffic in bytes (since boot)
}

impl NetworkAlertContext {
    pub fn build() -> NetworkAlertContext {
        let alert = monitor::get_alerts().unwrap();
        // stored wifi data values as bytes
        let stored_traffic = monitor::get_data().unwrap();
        let threshold = monitor::get_thresholds().unwrap();
        // current wifi traffic values as bytes
        let traffic = match network_client::traffic("wlan0") {
            Ok(t) => t,
            Err(_) => Traffic {
                received: 0,
                transmitted: 0,
                rx_unit: None,
                tx_unit: None,
            },
        };

        let current_traffic = traffic.received + traffic.transmitted;
        let total = stored_traffic.total + current_traffic;
        let data_total = Data { total };

        NetworkAlertContext {
            alert,
            back: None,
            data_total,
            flash_name: None,
            flash_msg: None,
            threshold,
            title: None,
            traffic,
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
    pub title: Option<String>,
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
        let wlan_ip = match network_client::ip("wlan0") {
            Ok(ip) => ip,
            Err(_) => "x.x.x.x".to_string(),
        };
        // list of networks saved in wpa_supplicant.conf
        let wlan_list = match network_client::saved_networks() {
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
        let saved_aps = match network_client::saved_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };
        let wlan_rssi = match network_client::rssi_percent("wlan0") {
            Ok(rssi) => Some(rssi),
            Err(_) => None,
        };
        // list of networks currently in range (online & accessible)
        let wlan_scan = match network_client::available_networks("wlan0") {
            Ok(networks) => {
                let scan: Vec<Scan> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                scan
            }
            Err(_) => Vec::new(),
        };
        let wlan_ssid = match network_client::ssid("wlan0") {
            Ok(ssid) => ssid,
            Err(_) => "Not connected".to_string(),
        };
        let wlan_state = match network_client::state("wlan0") {
            Ok(state) => state,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_status = match network_client::status("wlan0") {
            Ok(status) => status,
            Err(_) => "Interface unavailable".to_string(),
        };
        let wlan_traffic = match network_client::traffic("wlan0") {
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
            title: None,
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
    pub ap_state: String,
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
    pub wlan_networks: HashMap<String, String>,
    pub wlan_ssid: String,
}

impl NetworkListContext {
    pub fn build() -> NetworkListContext {
        // list of networks saved in the wpa_supplicant.conf
        let wlan_list = match network_client::saved_networks() {
            Ok(ssids) => {
                let networks: Vec<Networks> = serde_json::from_str(ssids.as_str())
                    .expect("Failed to deserialize scan_list response");
                networks
            }
            Err(_) => Vec::new(),
        };

        // list of networks currently in range (online & accessible)
        let wlan_scan = match network_client::available_networks("wlan0") {
            Ok(networks) => {
                let scan: Vec<Networks> = serde_json::from_str(networks.as_str())
                    .expect("Failed to deserialize scan_networks response");
                scan
            }
            Err(_) => Vec::new(),
        };

        let wlan_ssid = match network_client::ssid("wlan0") {
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

        NetworkListContext {
            ap_state,
            back: None,
            flash_msg: None,
            flash_name: None,
            title: None,
            wlan_networks,
            wlan_ssid,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PeerContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl PeerContext {
    pub fn build() -> PeerContext {
        PeerContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProfileContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl ProfileContext {
    pub fn build() -> ProfileContext {
        ProfileContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ShutdownContext {
    pub back: Option<String>,
    pub flash_name: Option<String>,
    pub flash_msg: Option<String>,
    pub title: Option<String>,
}

impl ShutdownContext {
    pub fn build() -> ShutdownContext {
        ShutdownContext {
            back: None,
            flash_name: None,
            flash_msg: None,
            title: None,
        }
    }
}
