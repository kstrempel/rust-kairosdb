extern crate kairosdb;
extern crate env_logger;

#[macro_use]
extern crate log;
use std::collections::HashMap;
use kairosdb::Client;
use kairosdb::datapoints::Datapoints;
use kairosdb::query::{Query, Metric, Time, TimeUnit};

#[test]
fn get_version() {
    let client = Client::new("localhost", 8080);
    assert!(client.version().unwrap().starts_with("KairosDB"));
}

#[test]
#[should_panic]
fn get_version_wrong_host() {
    let client = Client::new("www.google.com", 80);
    assert!(client.version().is_err());
    assert!(client.version().unwrap().starts_with("KairosDB"));
}

#[test]
fn add_datapoints() {
    let client = Client::new("localhost", 8080);
    let mut datapoints = Datapoints::new("first", 0);
    datapoints.add(1475513259000, 11);
    datapoints.add(1475513259001, 12);
    datapoints.add_tag("test", "first");
    let result = client.add(&datapoints);
    assert!(result.is_ok())
}

#[test]
fn query_empty() {
    let client = Client::new("localhost", 8080);
    let mut query = Query::new(
        Time::Absolute(1475513259000),
        Time::Relative{value: 5, unit: TimeUnit::DAYS});
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    tags.insert("test".to_string(), vec!["first".to_string()]);
    let mut metric = Metric::new("first",tags, vec![]);
    query.add(metric);
    let result = client.query(&query);
    println!("{:?}", result);
    assert!(result.is_ok())
}
