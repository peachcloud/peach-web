use std::fs::File;
use std::io::Read;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use super::rocket;
use crate::build_json_response;

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
fn test_index_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("PeachCloud Home"));
}

#[test]
fn test_network_card_html() {
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
fn test_network_list_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("WiFi Networks"));
    assert!(body.contains("No networks found"));
}

// TODO: needs further testing once template has been refactored
#[test]
fn test_network_detail_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/wifi?ssid=Home").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    //let body = response.body_string().unwrap();
    //assert!(body.contains("Network not found"));
}

#[test]
fn test_network_add_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi/add").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Add WiFi Network"));
    assert!(body.contains("SSID"));
    assert!(body.contains("Password"));
    assert!(body.contains("Connect"));
    assert!(body.contains("Cancel"));
}

#[test]
fn test_network_add_ssid_html() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/network/wifi/add?ssid=Home").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    let body = response.body_string().unwrap();
    assert!(body.contains("Add WiFi Network"));
    assert!(body.contains("Home"));
    assert!(body.contains("Password"));
    assert!(body.contains("Connect"));
    assert!(body.contains("Cancel"));
}

#[test]
fn test_add_credentials() {
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
fn test_forget_wifi() {
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
fn test_modify_password() {
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
fn test_deploy_ap() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/ap/activate").dispatch();
    // check for 303 status (redirect)
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

#[test]
fn test_deploy_client() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/network/wifi/activate").dispatch();
    // check for 303 status (redirect)
    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.content_type(), None);
}

// JSON API ROUTES

#[test]
fn test_activate_ap() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/api/v1/network/activate_ap")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn test_activate_client() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/api/v1/network/activate_client")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn test_return_ip() {
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
fn test_return_rssi() {
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
fn test_return_ssid() {
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
fn test_return_state() {
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
fn test_return_status() {
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
fn test_scan_networks() {
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
fn test_add_wifi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi")
        .header(ContentType::Form)
        .body("ssid=Home&pass=Password")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to add WiFi credentials."));
}

#[test]
fn test_remove_wifi() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi/forget")
        .header(ContentType::Form)
        .body("ssid=Home")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to retrieve network ID."));
}

#[test]
fn test_new_password() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/api/v1/network/wifi/modify")
        .header(ContentType::Form)
        .body("ssid=Home&pass=Password")
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let body = response.body_string().unwrap();
    assert!(body.contains("Failed to retrieve network ID."));
}

#[test]
fn test_ping_pong() {
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
fn test_nested_file() {
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
fn test_icon_file() {
    test_query_file(
        "/icons/peach-icon.png",
        "static/icons/peach-icon.png",
        Status::Ok,
    );
}

#[test]
fn test_invalid_path() {
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou_shalt_not_exist", None, Status::NotFound);
    test_query_file("/thou/shalt/not/exist?a=b&c=d", None, Status::NotFound);
}

#[test]
fn test_invalid_get_request() {
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
