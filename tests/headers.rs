extern crate hyper;
extern crate restson;

#[macro_use]
extern crate serde_derive;

use hyper::header::*;
use restson::{Error, RestClient, RestPath};

#[derive(Deserialize)]
struct HttpBinAnything {
    headers: TestHeaders,
}

#[derive(Deserialize)]
struct TestHeaders {
    #[serde(default)]
    #[serde(rename = "User-Agent")]
    user_agent: String,

    #[serde(default)]
    #[serde(rename = "X-Test")]
    test: String,
}

impl RestPath<()> for HttpBinAnything {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from("anything"))
    }
}

#[test]
fn headers() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    client
        .set_header(USER_AGENT.as_str(), "restson-test")
        .unwrap();

    let data: HttpBinAnything = client.get(()).unwrap();
    assert_eq!(data.headers.user_agent, "restson-test");
}

#[test]
fn headers_clear() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    client.set_header("X-Test", "12345").unwrap();

    let data: HttpBinAnything = client.get(()).unwrap();
    assert_eq!(data.headers.test, "12345");

    client.clear_headers();

    let data: HttpBinAnything = client.get(()).unwrap();
    assert_eq!(data.headers.test, "");
}

#[test]
fn default_user_agent() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    let data: HttpBinAnything = client.get(()).unwrap();
    assert_eq!(
        data.headers.user_agent,
        "restson/".to_owned() + env!("CARGO_PKG_VERSION")
    );
}
