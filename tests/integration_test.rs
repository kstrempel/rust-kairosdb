extern crate kairosdb;

use kairosdb::Client;
use kairosdb::datapoints::Datapoints;

#[test]
fn get_version() {
    let client = Client::new("localhost", 8080);
    assert!(client.version().unwrap().starts_with("KairosDB"));
}

#[test]
#[should_panic]
fn get_version_wrong_host() {
    let client = Client::new("www.google.com", 80);
    assert!(client.version().unwrap().starts_with("KairosDB"));
}

#[test]
fn add_datapoints() {
    let client = Client::new("localhost", 8080);
    let mut datapoints = Datapoints::new("first", 0);
    datapoints.add(1475513259000, 11);
    datapoints.add_tag("test", "first");
    let result = client.add(&datapoints);
    assert!(result.is_err())
}
