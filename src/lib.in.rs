use std::io::Read;
use std::time::Duration;
use hyper::header::Connection;
use hyper::status::StatusCode;
use datapoints::Datapoints;


pub struct Client {
    base_url: String,
    http_client: hyper::Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version: String,
}

impl Client {
    pub fn new(host: &str, port: u32) -> Client {
        let mut http_client = hyper::Client::new();
        http_client.set_read_timeout(Some(Duration::new(5, 0)));
        http_client.set_write_timeout(Some(Duration::new(5, 0)));
         Client {
            base_url: format!("http://{}:{}", host, port),
            http_client: http_client
        }
    }

    pub fn version(&self) -> Result<String, hyper::Error> {
        let mut response = self.http_client
            .get(&format!("{}/api/v1/version", self.base_url))
            .header(Connection::close())
            .send()
            .unwrap();
        let mut body = String::new();
        response.read_to_string(&mut body);
        let version: Version = serde_json::from_str(&body).unwrap();

        Ok(version.version)
    }

    pub fn add(&self, datapoints: &Datapoints) -> Result<(), hyper::Error> {
        let body = serde_json::to_string(datapoints).unwrap();
        let mut response = self.http_client
            .post(&format!("{}/api/v1/datapoints", self.base_url))
            .header(Connection::close())
            .body(&body)
            .send()
            .unwrap();
        let status = response.status == StatusCode::Ok;

        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::Client;
    use datapoints::Datapoints;

    #[test]
    fn get_version() {
        let client = Client::new("localhost", 8080);
        assert!(client.version().unwrap().starts_with("KairosDB"));
    }

    #[test]
    #[should_panic]
    fn get_version_wrong_host() {
        let client = Client::new("www.google.com", 80);
        client.version();
    }

    #[test]
    fn add_datapoints() {
        let client = Client::new("localhost", 8080);
        let datapoints = Datapoints::new("first", 0);
        assert!(match client.add(&datapoints) {
            Ok(_) => true,
            _ => false,
        });
    }
}
