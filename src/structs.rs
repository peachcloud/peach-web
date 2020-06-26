//use rocket::http::RawStr;
use rocket_contrib::json::JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessPoint {
    pub detail: Option<Scan>,
    pub signal: Option<i32>,
    pub state: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Traffic {
    pub received: u64,
    pub transmitted: u64,
    pub rx_unit: Option<String>,
    pub tx_unit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Uptime {
    pub secs: u64,
    pub nanos: u32,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}
