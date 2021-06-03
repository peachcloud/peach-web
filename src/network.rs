//! Provides network-related data structures.
//!
//! These particular structs are defined here, and not in `peach-lib`, because
//! they utilize macros from the Rocket crate. Incorporating these structs into
//! `peach-lib` would require adding Rocket as a dependency (undesirable).

use rocket::request::FromForm;
use rocket::UriDisplayQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromForm, UriDisplayQuery)]
pub struct Ssid {
    pub ssid: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct WiFi {
    pub ssid: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct DnsForm {
    pub external_domain: String,
    pub enable_dyndns: bool,
    pub dynamic_domain: String,
}