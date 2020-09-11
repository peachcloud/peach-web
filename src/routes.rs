//! Route handlers for PeachCloud web routes.
//!
//! This module contains handlers which serve templates and static assests,
//! generate flash messages, catch errors and handle redirects for PeachCloud.
//!
//! WEB ROUTES
//!
//! | Method | URL                         | Description                       |
//! | ------ | --------------------------- | --------------------------------- |
//! | GET    | /                           | Home                              |
//! | GET    | /device                     | Device statistics                 |
//! | GET    | /device/reboot              | Reboot device                     |
//! | GET    | /device/shutdown            | Shutdown device                   |
//! | GET    | /help                       | Help and usage guidelines         |
//! | GET    | /network                    | Network overview                  |
//! | GET    | /network/ap/activate        | Activate WiFi access point mode   |
//! | GET    | /network/wifi               | List of networks                  |
//! | GET    | /network/wifi?<ssid>        | Details of single network         |
//! | GET    | /network/wifi/activate      | Activate WiFi client mode         |
//! | GET    | /network/wifi/add           | Add WiFi form                     |
//! | POST   | /network/wifi/add           | WiFi form submission              |
//! | GET    | /network/wifi/add?<ssid>    | Add WiFi form (SSID populated)    |
//! | GET    | /network/wifi/alert         | WiFi data alerts form             |
//! | POST   | /network/wifi/alert         | WiFi data alerts form submission  |
//! | POST   | /network/wifi/connect       | Connect to WiFi access point      |
//! | POST   | /network/wifi/disconnect    | Disconnect from WiFi access point |
//! | POST   | /network/wifi/forget        | Remove WiFi                       |
//! | GET    | /network/wifi/modify?<ssid> | Modify WiFi password form         |
//! | POST   | /network/wifi/modify        | Modify network password           |
//! | GET    | /messages                   | Private Scuttlebutt messages      |
//! | GET    | /peers                      | Scuttlebutt peers overview        |
//! | GET    | /profile                    | Scuttlebutt user profile          |
//! | GET    | /shutdown                   | Shutdown menu                     |

use std::path::{Path, PathBuf};

use crate::context::{
    DeviceContext, FlashContext, HelpContext, MessageContext, NetworkAddContext,
    NetworkAlertContext, NetworkContext, NetworkDetailContext, NetworkListContext, PeerContext,
    ProfileContext,
};
use crate::device::*;
use crate::monitor::*;
use crate::network::*;
use crate::network_client::*;

use percent_encoding::percent_decode;

use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};

use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context = FlashContext {
        flash_name: None,
        flash_msg: None,
    };
    Template::render("index", &context)
}

#[get("/device")]
pub fn device_stats(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = DeviceContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("device", &context)
}

#[get("/device/reboot")]
pub fn reboot_cmd() -> Flash<Redirect> {
    match device_reboot() {
        Ok(_) => Flash::success(Redirect::to("/shutdown"), "Rebooting the device."),
        Err(_) => Flash::error(Redirect::to("/shutdown"), "Failed to reboot the device."),
    }
}

#[get("/device/shutdown")]
pub fn shutdown_cmd() -> Flash<Redirect> {
    match device_shutdown() {
        Ok(_) => Flash::success(Redirect::to("/shutdown"), "Shutting down the device."),
        Err(_) => Flash::error(Redirect::to("/shutdown"), "Failed to shutdown the device."),
    }
}

#[get("/help")]
pub fn help(flash: Option<FlashMessage>) -> Template {
    let mut context = HelpContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("help", &context)
}

#[get("/network")]
pub fn network(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_card", &context)
}

#[get("/network/ap/activate")]
pub fn deploy_ap() -> Flash<Redirect> {
    // activate the wireless access point
    debug!("Activating WiFi access point.");
    match network_activate_ap() {
        Ok(_) => Flash::success(Redirect::to("/network"), "Activated WiFi access point."),
        Err(_) => Flash::error(
            Redirect::to("/network"),
            "Failed to activate WiFi access point.",
        ),
    }
}

#[get("/network/wifi")]
pub fn wifi_list(flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkListContext::build();
    context.back = Some("/network".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_list", &context)
}

#[get("/network/wifi?<ssid>")]
pub fn network_detail(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
    // assign context through context_builder call
    let mut context = NetworkDetailContext::build();
    context.back = Some("/network/wifi".to_string());
    // decode ssid from url
    let decoded_ssid = percent_decode(ssid.as_bytes()).decode_utf8().unwrap();
    context.selected = Some(decoded_ssid.to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_detail", &context)
}

#[get("/network/wifi/activate")]
pub fn deploy_client() -> Flash<Redirect> {
    // activate the wireless client
    debug!("Activating WiFi client mode.");
    match network_activate_client() {
        Ok(_) => Flash::success(Redirect::to("/network"), "Activated WiFi client."),
        Err(_) => Flash::error(Redirect::to("/network"), "Failed to activate WiFi client."),
    }
}

#[get("/network/wifi/add")]
pub fn network_add_wifi(flash: Option<FlashMessage>) -> Template {
    let mut context = NetworkContext::build();
    // set back icon link to network route
    context.back = Some("/network".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_add", &context)
}

#[get("/network/wifi/add?<ssid>")]
pub fn network_add_ssid(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
    // decode ssid from url
    let decoded_ssid = percent_decode(ssid.as_bytes()).decode_utf8().unwrap();
    let mut context = NetworkAddContext {
        back: Some("/network/wifi".to_string()),
        selected: Some(decoded_ssid.to_string()),
        flash_name: None,
        flash_msg: None,
    };
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_add", &context)
}

#[post("/network/wifi/add", data = "<wifi>")]
pub fn add_credentials(wifi: Form<WiFi>) -> Template {
    // check if the credentials already exist for this access point
    // note: this is nicer but it's an unstable feature:
    //       if check_saved_aps(&wifi.ssid).contains(true)
    // use unwrap_or instead, set value to false if err is returned
    let creds_exist = check_saved_aps(&wifi.ssid).unwrap_or(false);
    if creds_exist {
        let context = FlashContext {
            flash_name: Some("error".to_string()),
            flash_msg: Some("Network credentials already exist for this access point".to_string()),
        };
        // return early from handler with "creds already exist" message
        return Template::render("network_add", &context);
    };

    // if credentials not found, generate and write wifi config to wpa_supplicant
    match network_add(&wifi.ssid, &wifi.pass) {
        Ok(_) => {
            debug!("Added WiFi credentials.");
            // force reread of wpa_supplicant.conf file with new credentials
            match network_reconfigure() {
                Ok(_) => debug!("Successfully reconfigured wpa_supplicant."),
                Err(_) => warn!("Failed to reconfigure wpa_supplicant."),
            }
            let context = FlashContext {
                flash_name: Some("success".to_string()),
                flash_msg: Some("Added WiFi credentials".to_string()),
            };
            Template::render("network_add", &context)
        }
        Err(_) => {
            debug!("Failed to add WiFi credentials.");
            let context = FlashContext {
                flash_name: Some("error".to_string()),
                flash_msg: Some("Failed to add WiFi credentials".to_string()),
            };
            Template::render("network_add", &context)
        }
    }
}

#[get("/network/wifi/usage")]
pub fn wifi_usage(flash: Option<FlashMessage>) -> Template {
    let mut context = NetworkAlertContext::build();
    // set back icon link to network route
    context.back = Some("/network".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_usage", &context)
}

#[post("/network/wifi/usage", data = "<thresholds>")]
pub fn wifi_usage_alerts(thresholds: Form<Threshold>) -> Flash<Redirect> {
    match update_store(thresholds.into_inner()) {
        Ok(_) => {
            debug!("WiFi data usage thresholds updated.");
            Flash::success(
                Redirect::to("/network/wifi/usage"),
                "Updated alert thresholds and flags",
            )
        }
        Err(_) => {
            warn!("Failed to update WiFi data usage thresholds.");
            Flash::error(
                Redirect::to("/network/wifi/usage"),
                "Failed to update alert thresholds and flags",
            )
        }
    }
}

#[post("/network/wifi/connect", data = "<network>")]
pub fn connect_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let ssid = &network.ssid;
    let url = uri!(network_detail: ssid);
    match network_id("wlan0", &ssid) {
        Ok(id) => match network_connect(&id, "wlan0") {
            Ok(_) => Flash::success(Redirect::to(url), "Connected to chosen network."),
            Err(_) => Flash::error(Redirect::to(url), "Failed to connect to chosen network."),
        },
        Err(_) => Flash::error(Redirect::to(url), "Failed to retrieve the network ID."),
    }
}

#[post("/network/wifi/disconnect", data = "<network>")]
pub fn disconnect_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let ssid = &network.ssid;
    let url = uri!(network);
    match network_disable("wlan0", &ssid) {
        Ok(_) => Flash::success(Redirect::to(url), "Disconnected from WiFi network."),
        Err(_) => Flash::error(Redirect::to(url), "Failed to disconnect from WiFi network."),
    }
}

#[post("/network/wifi/forget", data = "<network>")]
pub fn forget_wifi(network: Form<Ssid>) -> Flash<Redirect> {
    let ssid = &network.ssid;
    let url = uri!(network);
    match forget_network("wlan0", &ssid) {
        Ok(_) => Flash::success(Redirect::to(url), "WiFi credentials removed."),
        Err(_) => Flash::error(
            Redirect::to(url),
            "Failed to remove WiFi credentials".to_string(),
        ),
    }
}

#[get("/network/wifi/modify?<ssid>")]
pub fn wifi_password(ssid: &RawStr, flash: Option<FlashMessage>) -> Template {
    // decode ssid from url
    let decoded_ssid = percent_decode(ssid.as_bytes()).decode_utf8().unwrap();
    let mut context = NetworkAddContext {
        back: Some("/network/wifi".to_string()),
        selected: Some(decoded_ssid.to_string()),
        flash_name: None,
        flash_msg: None,
    };
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    // template_dir is set in Rocket.toml
    Template::render("network_modify", &context)
}

#[post("/network/wifi/modify", data = "<wifi>")]
pub fn wifi_set_password(wifi: Form<WiFi>) -> Flash<Redirect> {
    let ssid = &wifi.ssid;
    let pass = &wifi.pass;
    let url = uri!(network_detail: ssid);
    match update_password("wlan0", ssid, pass) {
        Ok(_) => Flash::success(Redirect::to(url), "WiFi password updated.".to_string()),
        Err(_) => Flash::error(
            Redirect::to(url),
            "Failed to update WiFi password".to_string(),
        ),
    }
}

#[get("/messages")]
pub fn messages(flash: Option<FlashMessage>) -> Template {
    let mut context = MessageContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("messages", &context)
}

#[get("/peers")]
pub fn peers(flash: Option<FlashMessage>) -> Template {
    let mut context = PeerContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("peers", &context)
}

#[get("/profile")]
pub fn profile(flash: Option<FlashMessage>) -> Template {
    let mut context = ProfileContext::build();
    context.back = Some("/".to_string());
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("profile", &context)
}

#[get("/shutdown")]
pub fn shutdown_menu(flash: Option<FlashMessage>) -> Template {
    let mut context = FlashContext {
        flash_name: None,
        flash_msg: None,
    };
    // check to see if there is a flash message to display
    if let Some(flash) = flash {
        // add flash message contents to the context object
        context.flash_name = Some(flash.name().to_string());
        context.flash_msg = Some(flash.msg().to_string());
    };
    Template::render("shutdown", &context)
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(404)]
pub fn not_found() -> Template {
    debug!("404 Page Not Found");
    // HACK: this is just here to satisfy the context requirement
    let context = FlashContext {
        flash_name: Some("error".to_string()),
        flash_msg: Some("No resource found for given URL".to_string()),
    };
    Template::render("not_found", context)
}

#[catch(500)]
pub fn internal_error() -> Template {
    debug!("500 Internal Server Error");
    // HACK: this is just here to satisfy the context requirement
    let context = FlashContext {
        flash_name: Some("error".to_string()),
        flash_msg: Some("Internal server error".to_string()),
    };
    Template::render("internal_error", context)
}
