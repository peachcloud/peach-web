//! Provides data structures which are used to parse forms from post requests.
//!
use rocket::request::FromForm;
use rocket::UriDisplayQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize, FromForm)]
pub struct DnsForm {
    pub external_domain: String,
    pub enable_dyndns: bool,
    pub dynamic_domain: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct PasswordForm {
    pub old_password: String,
    pub new_password1: String,
    pub new_password2: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct ResetPasswordForm {
    pub temporary_password: String,
    pub new_password1: String,
    pub new_password2: String,
}

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
pub struct AddAdminForm {
    pub ssb_id: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct DeleteAdminForm {
    pub ssb_id: String,
}
