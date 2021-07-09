//! This module contains core api functions shared by json_api.rs and by routes.rs
//!
//! These functions return Results which are then handled by the json api or the html routes
//! and turned into a rocket response appropriately.
use log::info;

use crate::error::PeachWebError;
use crate::forms::{DnsForm, PasswordForm, ResetPasswordForm};
use peach_lib::config_manager;
use peach_lib::dyndns_client;
use peach_lib::dyndns_client::{check_is_new_dyndns_domain, get_full_dynamic_domain};
use peach_lib::error::PeachError;
use peach_lib::jsonrpc_client_core::{Error, ErrorKind};
use peach_lib::jsonrpc_core::types::error::ErrorCode;
use peach_lib::password_utils;

pub fn save_dns_configuration(dns_form: DnsForm) -> Result<(), PeachWebError> {
    // first save local configurations
    config_manager::set_external_domain(&dns_form.external_domain)?;
    config_manager::set_dyndns_enabled_value(dns_form.enable_dyndns)?;
    // if dynamic dns is enabled and this is a new domain name, then register it
    if dns_form.enable_dyndns {
        let full_dynamic_domain = get_full_dynamic_domain(&dns_form.dynamic_domain);
        // check if this is a new domain or if its already registered
        let is_new_domain = check_is_new_dyndns_domain(&full_dynamic_domain);
        if is_new_domain {
            match dyndns_client::register_domain(&full_dynamic_domain) {
                Ok(_) => {
                    info!("Registered new dyndns domain");
                    // successful update
                    Ok(())
                }
                Err(err) => {
                    info!("Failed to register dyndns domain: {:?}", err);
                    // json response for failed update
                    let msg: String = match err {
                        PeachError::JsonRpcClientCore { source } => {
                            match source {
                                Error(ErrorKind::JsonRpcError(err), _state) => match err.code {
                                    ErrorCode::ServerError(-32030) => {
                                        format!("Error registering domain: {} was previously registered", full_dynamic_domain)
                                    }
                                    _ => {
                                        format!("Failed to register dyndns domain {:?}", err)
                                    }
                                },
                                _ => {
                                    format!("Failed to register dyndns domain: {:?}", source)
                                }
                            }
                        }
                        _ => "Failed to register dyndns domain".to_string(),
                    };
                    Err(PeachWebError::FailedToRegisterDynDomain { msg })
                }
            }
        }
        // if the domain is already registered, then dont re-register, and just return success
        else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

/// this function is for use by a user who is already logged in to change their password
pub fn save_password_form(password_form: PasswordForm) -> Result<(), PeachWebError> {
    info!(
        "change password!: {} {} {}",
        password_form.old_password, password_form.new_password1, password_form.new_password2
    );
    password_utils::verify_password(&password_form.old_password)?;
    // if the previous line did not throw an error, then the old password is correct
    password_utils::validate_new_passwords(
        &password_form.new_password1,
        &password_form.new_password2,
    )?;
    // if the previous line did not throw an error, then the new password is valid
    password_utils::set_new_password(&password_form.new_password1)?;
    Ok(())
}

/// this function is publicly exposed for users who have forgotten their password
pub fn save_reset_password_form(password_form: ResetPasswordForm) -> Result<(), PeachWebError> {
    info!(
        "reset password!: {} {} {}",
        password_form.temporary_password, password_form.new_password1, password_form.new_password2
    );
    password_utils::verify_temporary_password(&password_form.temporary_password)?;
    // if the previous line did not throw an error, then the secret_link is correct
    password_utils::validate_new_passwords(
        &password_form.new_password1,
        &password_form.new_password2,
    )?;
    // if the previous line did not throw an error, then the new password is valid
    password_utils::set_new_password(&password_form.new_password1)?;
    Ok(())
}
