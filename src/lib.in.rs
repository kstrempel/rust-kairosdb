use std::io::Read;
use hyper::header::Connection;
use hyper::status::StatusCode;
use datapoints::Datapoints;
use error::KairoError;

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version: String
}

#[derive(Debug)]
pub struct Client {
    base_url: String,
    http_client: hyper::Client
}

impl Client {
    pub fn new(host: &str, port: u32) -> Client {
        info!("create new client host: {} port: {}", host, port);
        Client {
            base_url: format!("http://{}:{}", host, port),
            http_client: hyper::Client::new(),
        }
    }

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

    pub fn add(&self, datapoints: &Datapoints) -> Result<(), KairoError> {
        info!("Add datapoints {:?}", datapoints);
        let body = try!(serde_json::to_string(&vec![datapoints]));
        let response = try!(self.http_client
            .post(&format!("{}/api/v1/datapoints", self.base_url))
            .header(Connection::close())
            .body(&body)
            .send());
        match response.status {
            StatusCode::Created => Ok(()),
            _ => {
                warn!("Add datapoints returns with bad response code: {:?}", response.status);
                Err(KairoError::KairoError("Bad response code".to_string()))
            }
        }
    }
}
