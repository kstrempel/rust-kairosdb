extern crate kairosdb;
extern crate env_logger;

#[macro_use]
extern crate log;
use std::collections::HashMap;
use kairosdb::Client;
use kairosdb::datapoints::Datapoints;
use kairosdb::query::{Query, Metric, Time, TimeUnit,
                      RelativeTime, Aggregator, AggregatorType};

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
    datapoints.add(1475513259000, 11.0);
    datapoints.add(1475513259001, 12.0);
    datapoints.add_tag("test", "first");
    let result = client.add(&datapoints);
    assert!(result.is_ok())
}

#[test]
fn simple_query() {
    let client = Client::new("localhost", 8080);

    let mut datapoints = Datapoints::new("second", 0);
    datapoints.add(1147724326001, 111.0);
    datapoints.add(1147724326040, 112.0);
    datapoints.add(1147724326051, 115.0);
    datapoints.add_tag("test", "second");
    let _ = client.add(&datapoints);

    let mut query = Query::new(
        Time::Absolute(1147724326000),
        Time::Absolute(1147724326040));

    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    tags.insert("test".to_string(), vec!["second".to_string()]);
    let metric = Metric::new("second",tags, vec![]);
    query.add(metric);

    let result = client.query(&query).unwrap();
    assert!(result.contains_key("second"));
    let first = &result["second"][0];
    assert_eq!(first.time, 1147724326001);
    assert_eq!(first.value, 111.0);

    let second = &result["second"][1];
    assert_eq!(second.time,1147724326040);
    assert_eq!(second.value, 112.0);

    let array = &result["second"];
    assert_eq!(array.len(), 2);
}

#[test]
fn metrics_average_query() {
    let client = Client::new("localhost", 8080);

    let mut datapoints = Datapoints::new("second", 0);
    datapoints.add(1147724326001, 111.0);
    datapoints.add(1147724326040, 112.0);
    datapoints.add(1147724326051, 115.0);
    datapoints.add_tag("test", "second");
    let _ = client.add(&datapoints);

    let mut query = Query::new(
        Time::Absolute(1147724326000),
        Time::Absolute(1147724326040));

    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    tags.insert("test".to_string(), vec!["second".to_string()]);
    let aggregator = Aggregator::new(AggregatorType::AVG, RelativeTime::new(10,TimeUnit::MINUTES));
    let metric = Metric::new("second",tags, vec![aggregator]);
    query.add(metric);

    let result = client.query(&query).unwrap();
    assert!(result.contains_key("second"));
    let first = &result["second"][0];
    assert_eq!(first.time, 1147724326001);
    assert_eq!(first.value, 111.5);

    let array = &result["second"];
    assert_eq!(array.len(), 1);
}
