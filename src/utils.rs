use regex::Regex;

use peach_lib::config_manager::load_peach_config;

/// helper function which builds a full dynamic dns domain from a subdomain
pub fn get_full_dynamic_domain(subdomain: &str) -> String {
    format!("{}.dyn.peachcloud.org", subdomain)
}

/// helper function to get a dyndns subdomain from a dyndns full domain
pub fn get_dyndns_subdomain(dyndns_full_domain: &str) -> String {
    let re = Regex::new(r"(.*)\.dyn\.peachcloud\.org").unwrap();
    let caps = re.captures(dyndns_full_domain).unwrap();
    let subdomain = caps.get(1).map_or("", |m| m.as_str());
    subdomain.to_string()
}

// helper function which checks if a dyndns domain is new
pub fn check_is_new_dyndns_domain(dyndns_full_domain: &str) -> bool {
    let peach_config = load_peach_config().unwrap();
    let previous_dyndns_domain = peach_config.dyn_domain;
    dyndns_full_domain != previous_dyndns_domain
}
