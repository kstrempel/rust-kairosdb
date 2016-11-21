/*
 * Copyright 2016 Kai Strempel
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
//! use kairosdb::query::{Query, Time, Metric, TimeUnit};
//!
//! let mut query = Query::new(
//!    Time::Nanoseconds(1000),
//!    Time::Nanoseconds(2000));
//!
//! let mut tags: HashMap<String, Vec<String>> = HashMap::new();
//! let metric = Metric::new("myMetric", tags, vec![]);
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
//! Deleting data is like querying data.
//!
//! ```
//! # fn main() {
//! # use kairosdb::Client;
//! # let client = Client::new("localhost", 8080);
//! use std::collections::HashMap;
//! use kairosdb::query::{Query, Time, TimeUnit, Metric};
//!
//! let mut query = Query::new(
//!    Time::Nanoseconds(1000),
//!    Time::Nanoseconds(2000));
//!
//! let mut tags: HashMap<String, Vec<String>> = HashMap::new();
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
//!
//! let result = client.metricnames();
//! assert!(result.unwrap().contains(&"myMetric".to_string()));
//! ```

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
