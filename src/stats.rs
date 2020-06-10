extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::StatsError;
use crate::structs::{CpuStatPercentages, LoadAverage, MemStat, Uptime};

// TODO: Replace unwraps with snafu errors

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
///
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
///
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
///
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
///
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
///
pub fn stats_ping() -> std::result::Result<MemStat, StatsError> {
    debug!("Creating HTTP transport for stats client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_STATS_SERVER").unwrap_or_else(|_| "127.0.0.1:5113".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_stats service.");
    let mut client = PeachStatsClient::new(transport_handle);

    let response = client.ping().call()?;
    let m: MemStat = serde_json::from_str(&response)?;

    Ok(m)
}

/// Creates a JSON-RPC client with http transport and calls the `peach-stats`
/// `cpu_stats_percent` method.
///
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
