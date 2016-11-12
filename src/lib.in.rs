use std::io::Read;

use hyper::header::Connection;
use hyper::status::StatusCode;

use datapoints::Datapoints;
use query::Query;
use result::{QueryResult, ResultMap};
use error::KairoError;

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
        let mut response = try!(self.http_client
            .get(&format!("{}/api/v1/version", self.base_url))
            .header(Connection::close())
            .send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        let version: Version = try!(serde_json::from_str(&body));

        info!("get server version {:?}", version.version);
        Ok(version.version)
    }

    /// Remote method to add datapoints to the time series database
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
        let body = try!(serde_json::to_string(&vec![datapoints]));
        let response = try!(self.http_client
            .post(&format!("{}/api/v1/datapoints", self.base_url))
            .header(Connection::close())
            .body(&body)
            .send());
        match response.status {
            StatusCode::NoContent=> Ok(()),
            _ => {
                let msg = format!("Add datapoints returns with bad response code: {:?}",
                                  response.status);
                Err(KairoError::Kairo(msg))
            }
        }
    }

    pub fn query(&self, query: &Query) -> Result<ResultMap, KairoError> {
        match self.run_query(query) {
            Ok(body) => self.parse_query_result(&body),
            Err(err) => Err(err)
        }
    }

    fn run_query(&self, query: &Query) -> Result<String, KairoError> {
        let body = try!(serde_json::to_string(query));
        info!("Run query {}", body);
        println!("Body {}", body);
        let mut response = try!(self.http_client
                                .post(&format!("{}/api/v1/datapoints/query",
                                               self.base_url))
                                .header(Connection::close())
                                .body(&body)
                                .send());

        match response.status {
            StatusCode::Ok => {
                let mut result_body = String::new();
                try!(response.read_to_string(&mut result_body));
                println!("Response {}", result_body);
                Ok(result_body)
            },
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
