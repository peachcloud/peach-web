//! Perform JSON-RPC calls to the `peach-stats` microservice.
//!
//! This module contains a JSON-RPC client and associated data structures for
//! making calls to the `peach-stats` microservice. Each RPC has a corresponding
//! method which creates an HTTP transport, makes the call to the RPC
//! microservice and returns the response to the caller. These convenience
//! methods simplify the process of performing RPC calls from other modules.

extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::StatsError;

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

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
pub fn cpu_stats_percent() -> std::result::Result<CpuStatPercentages, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.cpu_stats_percent().call()?;
    let c: CpuStatPercentages = serde_json::from_str(&response)?;

    Ok(c)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `disk_usage` method.
pub fn disk_usage() -> std::result::Result<String, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.disk_usage().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
pub fn load_average() -> std::result::Result<LoadAverage, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.load_average().call()?;
    let l: LoadAverage = serde_json::from_str(&response)?;

    Ok(l)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
pub fn mem_stats() -> std::result::Result<MemStat, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.mem_stats().call()?;
    let m: MemStat = serde_json::from_str(&response)?;

    Ok(m)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `ping` method.
pub fn stats_ping() -> std::result::Result<String, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.ping().call()?;

    Ok(response)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `uptime` method. If a successful response is returned, the uptime value (in
/// seconds) is converted to minutes before being returned to the caller.
pub fn uptime() -> std::result::Result<String, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.uptime().call()?;
    let u: Uptime = serde_json::from_str(&response)?;
    let minutes = (u.secs / 60).to_string();

    Ok(minutes)
}

jsonrpc_client!(pub struct PeachStatsClient {
    /// JSON-RPC request to get measurement of current CPU statistics.
    pub fn cpu_stats_percent(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current disk usage statistics.
    pub fn disk_usage(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current load average statistics.
    pub fn load_average(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get measurement of current memory statistics.
    pub fn mem_stats(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to check availability of the `peach-stats` microservice.
    pub fn ping(&mut self) -> RpcRequest<String>;

    /// JSON-RPC request to get system uptime.
    pub fn uptime(&mut self) -> RpcRequest<String>;
});
