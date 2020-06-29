extern crate jsonrpc_client_http;

use std::env;

use jsonrpc_client_http::HttpTransport;

use crate::error::MenuError;

pub fn oled_ping() -> std::result::Result<(), MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.ping().call()?;
    debug!("Pinged the OLED microservice.");

    Ok(())
}

jsonrpc_client!(pub struct PeachOledClient {
    /// Creates a JSON-RPC request to ping the OLED microservice.
    pub fn ping(&mut self) -> RpcRequest<String>;
});

/*
 * These methods are not currently in use but may be required later
 *
/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `clear`, `flush` and `write` methods.
///
/// # Arguments
///
/// * `x_coord` - A 32 byte signed int.
/// * `y_coord` - A 32 byte signed int.
/// * `string` - A String containing the message to be displayed.
/// * `font_size` - A String containing `6x8`, `6x12`, `8x16` or `12x16`
pub fn oled_clear() -> std::result::Result<(), MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.clear().call()?;
    debug!("Cleared the OLED display.");

    Ok(())
}

pub fn oled_draw(
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    x_coord: i32,
    y_coord: i32,
) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.draw(bytes, width, height, x_coord, y_coord).call()?;
    debug!("Drew to the OLED display.");

    Ok("success".to_string())
}

pub fn oled_flush() -> std::result::Result<(), MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.flush().call()?;
    debug!("Flushed the OLED display.");

    Ok(())
}

pub fn oled_power(power: bool) -> std::result::Result<(), MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.power(power).call()?;
    debug!("Toggled the OLED display power.");

    Ok(())
}

pub fn oled_write(
    x_coord: i32,
    y_coord: i32,
    string: &str,
    font_size: &str,
) -> std::result::Result<String, MenuError> {
    debug!("Creating HTTP transport for OLED client.");
    let transport = HttpTransport::new().standalone()?;
    let http_addr = env::var("PEACH_OLED_SERVER").unwrap_or_else(|_| "127.0.0.1:5112".to_string());
    let http_server = format!("http://{}", http_addr);
    debug!("Creating HTTP transport handle on {}.", http_server);
    let transport_handle = transport.handle(&http_server)?;
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    client.write(x_coord, y_coord, string, font_size).call()?;
    debug!("Wrote to the OLED display.");

    Ok("success".to_string())
}

    /// Creates a JSON-RPC request to clear the OLED display.
    pub fn clear(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to draw to the OLED display.
    pub fn draw(&mut self, bytes: Vec<u8>, width: u32, height: u32, x_coord: i32, y_coord: i32) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to flush the OLED display.
    pub fn flush(&mut self) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to toggle the power of the OLED display.
    pub fn power(&mut self, power: bool) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to write to the OLED display.
    pub fn write(&mut self, x_coord: i32, y_coord: i32, string: &str, font_size: &str) -> RpcRequest<String>;
 *
 */
