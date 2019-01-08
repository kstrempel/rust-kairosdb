// Copyright 2016-2017 Kai Strempel
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
extern crate serde_derive;
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
use std::io::Read;

use hyper::StatusCode;

use datapoints::Datapoints;
use query::Query;
use result::{QueryResult, ResultMap};
use error::KairoError;
use helper::parse_metricnames_result;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version: String,
}

/// The core of the kairosdb client, owns a HTTP connection.
#[derive(Debug)]
pub struct Client {
    base_url: String,
    http_client: hyper::Client<hyper::client::HttpConnector>
}

impl Client {
    /// Constructs a new KairosDB Client
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// let client = Client::new("localhost", 8080);
    /// ```
    pub fn new(host: &str, port: u32) -> Client {
        info!("create new client host: {} port: {}", host, port);
        Client {
            base_url: format!("http://{}:{}", host, port),
            http_client: hyper::Client::new(),
        }
    }

    /// Returns the version string of the KairosDB Server
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// let client = Client::new("localhost", 8080);
    /// assert!(client.version().unwrap().starts_with("KairosDB"));
    /// ```
    pub fn version(&self) -> Result<String, KairoError> {
        let mut response = self.http_client
            .get(&format!("{}/api/v1/version", self.base_url))
            .send()?;
        let mut body = String::new();
        response.read_to_string(&mut body)?;
        let version: Version = serde_json::from_str(&body)?;

        info!("get server version {:?}", version.version);
        Ok(version.version)
    }

    /// Returns the health status of the KairosDB Server
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// let client = Client::new("localhost", 8080);
    /// let response = client.health();
    /// ```
    pub fn health(&self) -> Result<Vec<String>, KairoError> {
        let mut response = self.http_client
            .get(&format!("{}/api/v1/health/status", self.base_url))
            .header(Connection::close())
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                let health: Vec<String> = serde_json::from_str(&body)?;
                info!("get server health {:?}", health);
                Ok(health)
            }
            _ => {
                let msg = format!("Health endpoint returns with wrong status code: {:?}",
                                  response.status);
                Err(KairoError::Kairo(msg))
            }
        }
    }

    /// Method to add datapoints to the time series database
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// use kairosdb::datapoints::Datapoints;
    ///
    /// let client = Client::new("localhost", 8080);
    /// let mut datapoints = Datapoints::new("first", 0);
    /// datapoints.add_ms(1475513259000, 11.0);
    /// datapoints.add_ms(1475513259001, 12.0);
    /// datapoints.add_tag("test", "first");
    /// let result = client.add(&datapoints);
    /// assert!(result.is_ok())
    /// ```
    pub fn add(&self, datapoints: &Datapoints) -> Result<(), KairoError> {
        info!("Add datapoints {:?}", datapoints);
        let body = serde_json::to_string(&vec![datapoints])?;
        let response = self.http_client
            .post(&format!("{}/api/v1/datapoints", self.base_url))
            .header(Connection::close())
            .body(&body)
            .send()?;
        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => {
                let msg = format!("Add datapoints returns with bad response code: {:?}",
                                  response.status);
                Err(KairoError::Kairo(msg))
            }
        }
    }

    /// Runs a query on the database.
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// use kairosdb::query::{Query, Time, TimeUnit};
    ///
    /// let client = Client::new("localhost", 8080);
    /// let query = Query::new(
    ///    Time::Nanoseconds(1),
    ///    Time::Relative{value: 1, unit: TimeUnit::WEEKS});
    /// let result = client.query(&query);
    /// assert!(result.is_ok())
    /// ```
    pub fn query(&self, query: &Query) -> Result<ResultMap, KairoError> {
        match self.run_query(query, "query") {
            Ok(body) => self.parse_query_result(&body),
            Err(err) => Err(err),
        }
    }

    /// Runs a delete query on the database. View the query structure
    /// to understand more about.
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// use kairosdb::query::{Query, Time, TimeUnit};
    ///
    /// let client = Client::new("localhost", 8080);
    /// let query = Query::new(
    ///    Time::Nanoseconds(1),
    ///    Time::Relative{value: 1, unit: TimeUnit::WEEKS});
    /// let result = client.delete(&query);
    /// assert!(result.is_ok())
    /// ```
    pub fn delete(&self, query: &Query) -> Result<(), KairoError> {
        match self.run_query(query, "delete") {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Returns a list with all metric names
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// # use kairosdb::datapoints::Datapoints;
    /// let client = Client::new("localhost", 8080);
    /// # let mut datapoints = Datapoints::new("first", 0);
    /// # datapoints.add_ms(1475513259000, 11.0);
    /// # datapoints.add_tag("test", "first");
    /// # let result = client.add(&datapoints);
    ///
    /// let result = client.list_metrics();
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().contains(&"first".to_string()));
    /// ```
    pub fn list_metrics(&self) -> Result<Vec<String>, KairoError> {
        info!("Get metricnames");
        let mut response = self.http_client
            .get(&format!("{}/api/v1/metricnames", self.base_url))
            .header(Connection::close())
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                response.read_to_string(&mut result_body)?;
                Ok(parse_metricnames_result(&result_body)?)
            }
            _ => Err(KairoError::Kairo(format!("Bad response code: {:?}", response.status))),
        }
    }

    /// Deleting a metric
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// # use kairosdb::datapoints::Datapoints;
    /// let client = Client::new("localhost", 8080);
    /// # let mut datapoints = Datapoints::new("first", 0);
    /// # datapoints.add_ms(1475513259000, 11.0);
    /// # let result = client.add(&datapoints);
    ///
    /// let result = client.delete_metric(&"first");
    /// assert!(result.is_ok());
    /// # let result = client.list_metrics();
    /// # assert!(result.is_ok());
    /// # assert!(!result.unwrap().contains(&"first".to_string()));
    /// ```
    pub fn delete_metric(&self, metric: &str) -> Result<(), KairoError> {
        let response = self.http_client
            .delete(&format!("{}/api/v1/metric/{}", self.base_url, metric))
            .header(Connection::close())
            .send()?;

        match response.status {
            StatusCode::NoContent => Ok(()),
            _ => Err(KairoError::Kairo(format!("Bad response code: {:?}", response.status))),
        }
    }

    /// Returns a list of all tagnames
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// # use kairosdb::datapoints::Datapoints;
    /// let client = Client::new("localhost", 8080);
    /// # let mut datapoints = Datapoints::new("first", 0);
    /// # datapoints.add_ms(1475513259000, 11.0);
    /// # datapoints.add_tag("test", "first");
    /// # let _ = client.add(&datapoints);
    ///
    /// let result = client.tagnames();
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().contains(&"test".to_string()));
    /// ```
    pub fn tagnames(&self) -> Result<Vec<String>, KairoError> {
        info!("Get tagnames");
        let mut response = self.http_client
            .get(&format!("{}/api/v1/tagnames", self.base_url))
            .header(Connection::close())
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                response.read_to_string(&mut result_body)?;
                Ok(parse_metricnames_result(&result_body)?)
            }
            _ => Err(KairoError::Kairo(format!("Bad response code: {:?}", response.status))),
        }
    }

    /// Returns a list of all tagvalues
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// # use kairosdb::datapoints::Datapoints;
    /// let client = Client::new("localhost", 8080);
    /// # let mut datapoints = Datapoints::new("first", 0);
    /// # datapoints.add_ms(1475513259000, 11.0);
    /// # datapoints.add_tag("test", "first");
    /// # let _ = client.add(&datapoints);
    ///
    /// let result = client.tagvalues();
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().contains(&"first".to_string()));
    /// ```
    pub fn tagvalues(&self) -> Result<Vec<String>, KairoError> {
        info!("Get tagnames");
        let mut response = self.http_client
            .get(&format!("{}/api/v1/tagvalues", self.base_url))
            .header(Connection::close())
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                response.read_to_string(&mut result_body)?;
                Ok(parse_metricnames_result(&result_body)?)
            }
            _ => Err(KairoError::Kairo(format!("Bad response code: {:?}", response.status))),
        }
    }

    fn run_query(&self, query: &Query, endpoint: &str) -> Result<String, KairoError> {
        let body = serde_json::to_string(query)?;
        info!("Run query {}", body);
        let mut response = self.http_client
            .post(&format!("{}/api/v1/datapoints/{}", self.base_url, endpoint))
            .header(Connection::close())
            .body(&body)
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                response.read_to_string(&mut result_body)?;
                Ok(result_body)
            }
            StatusCode::NoContent => Ok("".to_string()),
            _ => Err(KairoError::Kairo(format!("Bad response code: {:?}", response.status))),
        }
    }


    fn parse_query_result(&self, body: &str) -> Result<ResultMap, KairoError> {
        let result = QueryResult::new();
        result.parse_result(body)
    }
}
