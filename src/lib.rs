// Copyright 2016 Kai Strempel
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#![crate_type = "lib"]
#![crate_name = "kairosdb"]

//! A `Client` for KairosBD REST API
//!
//! The Client itself is used as the central access point, from which
//! numerous operations are defined implementing each of the specific
//! KairosDB APIs.
//!
//! ```
//! use kairosdb::Client;
//! let client = Client::new("localhost", 8080);
//! ```
//!
//! A main job of a time series database is collecting and querying data.
//! To add data to KairosDB we have to create a `Datapoints` struct and add
//! the data to the object.
//!
//! ```
//! # fn main() {
//! # use kairosdb::Client;
//! use kairosdb::datapoints::Datapoints;
//! # let client = Client::new("localhost", 8080);
//!
//! let mut datapoints = Datapoints::new("myMetric", 0);
//! datapoints.add_ms(1000, 11.0);
//! datapoints.add_ms(2000, 12.0);
//! datapoints.add_ms(3000, 13.0);
//! datapoints.add_tag("test", "first");
//! let result = client.add(&datapoints);
//! assert!(result.is_ok());
//! # }
//! ```
//!
//! To query data we have to create a `Query` Object with the start and end
//! of the query. The start and the end can be a relative time. Check the
//! 'Time' structure for more information.
//!
//! ```
//! # fn main() {
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! use std::collections::HashMap;
//! use kairosdb::query::{Query, Time, Metric, Tags};
//! # use kairosdb::datapoints::Datapoints;
//! # let mut datapoints = Datapoints::new("myMetric", 0);
//! # datapoints.add_ms(1000, 11.0);
//! # datapoints.add_ms(2000, 12.0);
//! # datapoints.add_ms(3000, 13.0);
//! # datapoints.add_tag("test", "first");
//! # let result = client.add(&datapoints);
//! # assert!(result.is_ok());
//!
//! let mut query = Query::new(
//!    Time::Nanoseconds(1000),
//!    Time::Nanoseconds(2000));
//!
//! let metric = Metric::new("myMetric", Tags::new(), vec![]);
//! query.add(metric);
//!
//! let result = client.query(&query).unwrap();
//!
//! assert!(result.contains_key("myMetric"));
//! assert_eq!(result["myMetric"].len(), 2);
//! assert_eq!(result["myMetric"][0].time, 1000);
//! assert_eq!(result["myMetric"][0].value, 11.0);
//! assert_eq!(result["myMetric"][1].time, 2000);
//! assert_eq!(result["myMetric"][1].value, 12.0);
//! # }
//! ```
//!
//! Optionally you can specify aggregators. Aggregators perform an operation on data
//! points. For example, you can sum all data points that exist in
//! 5 minute periods. Aggregators can be combined together. E.g you could
//! sum all data points in 5 minute periods then calculate the average of them for a
//! week period.
//! Aggregators are processed in the order they are specified in the vector for the
//! metric constructor.
//!
//! ```
//! # fn main() {
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! use kairosdb::query::*;
//! use kairosdb::datapoints::Datapoints;
//! # let result = client.delete_metric(&"myMetric");
//! # assert!(result.is_ok());
//! for i in 0..10 {
//!    let mut datapoints = Datapoints::new("myMetric", 0);
//!    datapoints.add_ms(i * 500, i as f64);
//!    datapoints.add_tag("test", "first");
//!    let result = client.add(&datapoints);
//!    assert!(result.is_ok());
//! }
//!
//! let mut query = Query::new(
//!    Time::Nanoseconds(0),
//!    Time::Nanoseconds(10*500));
//!
//! let aggregator = Aggregator::new(
//!     AggregatorType::AVG,
//!     RelativeTime::new(1, TimeUnit::SECONDS));
//! let metric = Metric::new("myMetric", Tags::new(), vec![aggregator]);
//! query.add(metric);
//!
//! let result = client.query(&query).unwrap();
//! assert!(result.contains_key("myMetric"));
//! assert_eq!(result["myMetric"].len(), 5);
//! assert_eq!(result["myMetric"][0].time, 0);
//! assert_eq!(result["myMetric"][0].value, 0.5);
//! # assert_eq!(result["myMetric"][1].time, 1000);
//! # assert_eq!(result["myMetric"][1].value, 2.5);
//! # assert_eq!(result["myMetric"][2].time, 2000);
//! # assert_eq!(result["myMetric"][2].value, 4.5);
//! # assert_eq!(result["myMetric"][3].time, 3000);
//! # assert_eq!(result["myMetric"][3].value, 6.5);
//! # assert_eq!(result["myMetric"][4].time, 4000);
//! # assert_eq!(result["myMetric"][4].value, 8.5);
//! # }
//! ```
//!
//! Deleting data is like querying data.
//!
//! ```
//! # fn main() {
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! use kairosdb::query::{Query, Time, Metric, Tags};
//!
//! let mut query = Query::new(
//!    Time::Nanoseconds(1000),
//!    Time::Nanoseconds(2000));
//!
//! let mut tags = Tags::new();
//! tags.insert("test".to_string(), vec!["first".to_string()]);
//! let metric = Metric::new("myMetric", tags, vec![]);
//! query.add(metric);
//!
//! let result = client.delete(&query);
//! assert!(result.is_ok());
//! # }
//! ```
//!
//! Getting the current set of metric names is a simple
//! function call.
//!
//! ```
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! # use kairosdb::datapoints::Datapoints;
//! # let mut datapoints = Datapoints::new("myMetric", 0);
//! # datapoints.add_ms(1000, 11.0);
//! # datapoints.add_ms(2000, 12.0);
//! # datapoints.add_ms(3000, 13.0);
//! # datapoints.add_tag("test", "first");
//! # let result = client.add(&datapoints);
//! # assert!(result.is_ok());
//!
//! let result = client.list_metrics();
//! assert!(result.unwrap().contains(&"myMetric".to_string()));
//! ```
//!
//! To get information about the current tags and tag values you
//! can use the tagsnames and tagvalues method.
//!
//! ```
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! # use kairosdb::datapoints::Datapoints;
//! # let mut datapoints = Datapoints::new("myMetric", 0);
//! # datapoints.add_tag("test", "first");
//! # let result = client.add(&datapoints);
//! # assert!(result.is_ok());
//!
//! let tagnames = client.tagnames();
//! let tagvalues = client.tagvalues();
//! assert!(tagnames.unwrap().contains(&"test".to_string()));
//! assert!(tagvalues.unwrap().contains(&"first".to_string()));
//! ```
//! Delete a metric by name
//!
//! ```
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//!
//! let result = client.delete_metric(&"myMetric");
//! assert!(result.is_ok());
//! ```
//!
//! ## Server status
//!
//! To get the health status of the KairosDB Server
//!
//! ```
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! let response = client.health();
//! let result = response.unwrap();
//! assert_eq!(result[0], "JVM-Thread-Deadlock: OK");
//! assert_eq!(result[1], "Datastore-Query: OK");
//! ```
//!
//! Get the version of the KairosDB Server
/// ```
/// # use kairosdb::Client;
/// let client = Client::new("localhost", 8080);
/// assert!(client.version().unwrap().starts_with("KairosDB"));
/// ```


extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate chrono;

pub mod datapoints;
pub mod query;
pub mod result;
mod error;
mod helper;

#[cfg(feature = "serde_macros")]
include!("lib.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
