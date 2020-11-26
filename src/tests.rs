use std::fs::File;
use std::io::Read;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use super::rocket;
use crate::json_api::build_json_response;

// helper function to test correct retrieval and content of a file
fn test_query_file<T>(path: &str, file: T, status: Status)
where
    T: Into<Option<&'static str>>,
{
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get(path).dispatch();
    assert_eq!(response.status(), status);

    let body_data = response.body().and_then(|body| body.into_bytes());
    if let Some(filename) = file.into() {
        let expected_data = read_file_content(filename);
        assert!(body_data.map_or(false, |s| s == expected_data));
    }
}

// helper function to return the content of a file, given a path
fn read_file_content(path: &str) -> Vec<u8> {
    let mut fp = File::open(&path).expect(&format!("Can't open {}", path));
    let mut file_content = vec![];

    fp.read_to_end(&mut file_content)
        .expect(&format!("Reading {} failed.", path));
    file_content
}

// WEB PAGE ROUTES

#[test]
fn index_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("/peers"));
    assert!(body.contains("/profile"));
    assert!(body.contains("/messages"));
    assert!(body.contains("/device"));
    assert!(body.contains("/help"));
    assert!(body.contains("/network"));
}

#[test]
fn network_card_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("MODE"));
    assert!(body.contains("SSID"));
    assert!(body.contains("IP"));
    assert!(body.contains("Add WiFi Network"));
    assert!(body.contains("Deploy Access Point"));
    assert!(body.contains("List WiFi Networks"));
    assert!(body.contains("SIGNAL"));
    assert!(body.contains("DOWNLOAD"));
    assert!(body.contains("UPLOAD"));
}

#[test]
fn network_list_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("WiFi Networks"));
    assert!(body.contains("No saved or available networks found."));
}

// TODO: needs further testing once template has been refactored
#[test]
fn network_detail_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/wifi?ssid=Home").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    //let body = response.body_string().unwrap();
    //assert!(body.contains("Network not found"));
}

#[test]
fn network_add_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi/add").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Add WiFi Network"));
    assert!(body.contains("SSID"));
    assert!(body.contains("Password"));
    assert!(body.contains("Add"));
    assert!(body.contains("Cancel"));
}

#[test]
fn network_add_ssid_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi/add?ssid=Home").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Add WiFi Network"));
    assert!(body.contains("Home"));
    assert!(body.contains("Password"));
    assert!(body.contains("Add"));
    assert!(body.contains("Cancel"));
}

#[test]
fn device_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/device").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Device Status"));
    assert!(body.contains("Networking"));
    assert!(body.contains("Display"));
    assert!(body.contains("Statistics"));
}

#[test]
fn help_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/help").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Help"));
}

#[test]
fn login_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/login").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Login"));
}

#[test]
fn messages_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/messages").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Private Messages"));
}

#[test]
fn peers_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/peers").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Scuttlebutt Peers"));
}

#[test]
fn profile_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/profile").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Profile"));
}

#[test]
fn shutdown_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/shutdown").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Shutdown Device"));
}

#[test]
fn network_usage_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi/usage").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Network Data Usage"));
    assert!(body.contains("WARNING THRESHOLD"));
    assert!(body.contains("Update"));
    assert!(body.contains("Cancel"));
}

#[test]
fn add_credentials() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/network/wifi/add")
        .header(ContentType::Form)
        .body("ssid=Home&pass=Password")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
}

#[test]
fn forget_wifi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/network/wifi/forget")
        .header(ContentType::Form)
        .body("ssid=Home")
        .dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

#[test]
fn modify_password() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/network/wifi/modify")
        .header(ContentType::Form)
        .body("ssid=Home&pass=Password")
        .dispatch();
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

#[test]
fn deploy_ap() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/ap/activate").dispatch();
    // check for 303 status (redirect)
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

#[test]
fn deploy_client() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/wifi/activate").dispatch();
    // check for 303 status (redirect)
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

// JSON API ROUTES

#[test]
fn activate_ap() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/api/v1/network/activate_ap")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn activate_client() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/api/v1/network/activate_client")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn return_ip() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/ip")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("wlan0"));
    assert!(body.contains("ap0"));
}

#[test]
fn return_rssi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/rssi")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Not currently connected to an access point."));
}

#[test]
fn return_ssid() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/ssid")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Not currently connected to an access point."));
}

#[test]
fn return_state() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/state")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("ap0"));
    assert!(body.contains("wlan0"));
    assert!(body.contains("unavailable"));
}

#[test]
fn return_status() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/status")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Not currently connected to an access point."));
}

#[test]
fn scan_networks() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/network/wifi")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Unable to scan for networks. Interface may be deactivated."));
}

#[test]
fn add_wifi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi")
        .header(ContentType::JSON)
        .body(r#"{ "ssid": "Home", "pass": "Password" }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to add WiFi credentials."));
}

#[test]
fn remove_wifi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi/forget")
        .header(ContentType::JSON)
        .body(r#"{ "ssid": "Home" }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to remove WiFi network credentials."));
}

#[test]
fn new_password() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi/modify")
        .header(ContentType::JSON)
        .body(r#"{ "ssid": "Home", "pass": "Password" }"#)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to update WiFi password."));
}

#[test]
fn ping_pong() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .get("/api/v1/ping")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("pong!"));
}

// HELPER FUNCTION TESTS

#[test]
fn test_build_json_response() {
    let status = "success".to_string();
    let data = json!("WiFi credentials added.".to_string());
    let json = build_json_response(status, Some(data), None);
    assert_eq!(json.status, "success");
    assert_eq!(json.data, Some(json!("WiFi credentials added.")));
    assert_eq!(json.msg, None);
}

// FILE TESTS

#[test]
fn nested_file() {
    test_query_file(
        "/images/placeholder.txt",
        "static/images/placeholder.txt",
        Status::Ok,
    );
    test_query_file(
        "/images/placeholder.txt?v=1",
        "static/images/placeholder.txt",
        Status::Ok,
    );
    test_query_file(
        "/images/placeholder.txt?v=1&a=b",
        "static/images/placeholder.txt",
        Status::Ok,
    );
}

#[test]
fn icon_file() {
    test_query_file(
        "/icons/peach-icon.png",
        "static/icons/peach-icon.png",
        Status::Ok,
    );
}

#[test]
fn invalid_path() {
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist?a=b&c=d", None, Status::NotFound);
}

#[test]
fn invalid_get_request() {
    let client = Client::new(rocket()).unwrap();

    // try to get a path that doesn't exist
    let mut res = client
        .get("/message/99")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("404: Page Not Found"));
    assert!(body.contains("No PeachCloud resource exists for this URL."));
}
