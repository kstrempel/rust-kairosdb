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

use std::io::Read;

use hyper::header::Connection;
use hyper::status::StatusCode;

use datapoints::Datapoints;
use query::Query;
use result::{QueryResult, ResultMap};
use error::KairoError;
use helper::parse_metricnames_result;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version: String,
}

#[derive(Debug)]
pub struct Client {
    base_url: String,
    http_client: hyper::Client,
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
            .header(Connection::close())
            .send()?;
        let mut body = String::new();
        response.read_to_string(&mut body)?;
        let version: Version = serde_json::from_str(&body)?;

        info!("get server version {:?}", version.version);
        Ok(version.version)
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
            StatusCode::NoContent=> Ok(()),
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
            Err(err) => Err(err)
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
            Err(err) => Err(err)
        }
    }

    /// Returns a list with all metric names
    ///
    /// # Example
    /// ```
    /// use kairosdb::Client;
    /// let client = Client::new("localhost", 8080);
    ///
    /// let result = client.metricnames();
    /// assert!(result.is_ok());
    /// assert!(result.unwrap().contains(&"first".to_string()));
    /// ```
    pub fn metricnames(&self) -> Result<Vec<String>, KairoError> {
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
            },
            _ => {
                Err(KairoError::Kairo(
                    format!("Bad response code: {:?}", response.status)))
            }
        }
    }

    fn run_query(&self,
                 query: &Query,
                 endpoint: &str) -> Result<String, KairoError> {
        let body = serde_json::to_string(query)?;
        info!("Run query {}", body);
        let mut response = self.http_client
            .post(&format!("{}/api/v1/datapoints/{}",
                           self.base_url,
                           endpoint))
            .header(Connection::close())
            .body(&body)
            .send()?;

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                response.read_to_string(&mut result_body)?;
                Ok(result_body)
            },
            StatusCode::NoContent => {
               Ok("".to_string())
            }
            _ => {
                Err(KairoError::Kairo(
                    format!("Bad response code: {:?}", response.status)))
            }
        }
    }


    fn parse_query_result(&self, body: &str) -> Result<ResultMap,
                                                       KairoError> {
        let result = QueryResult::new();
        result.parse_result(body)
    }
}
