use std::fs::File;
use std::io::Read;

use rocket::http::{ContentType, Status};
use rocket::local::Client;

use super::rocket;

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

#[test]
fn test_index_response() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_index_html() {
    test_query_file("/", "static/index.html", Status::Ok);
    test_query_file("/?v=1", "static/index.html", Status::Ok);
    test_query_file("/?this=should&be=ignored", "static/index.html", Status::Ok);
}

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
    test_query_file("/peach-icon.png", "static/peach-icon.png", Status::Ok);
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
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found"));
}
