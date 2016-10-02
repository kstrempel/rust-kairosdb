use std::io::Read;
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
        Client {
            base_url: format!("http://{}:{}", host, port),
            http_client: hyper::Client::new(),
        }
    }

    pub fn version(&self) -> Result<String, hyper::Error> {
        let mut response = try!(self.http_client
            .get(&format!("{}/api/v1/version", self.base_url))
            .send());
        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        let version: Version = serde_json::from_str(&body).unwrap();

        Ok(version.version)
    }

    pub fn add(&self, datapoints: &Datapoints) -> Result<(), hyper::Error> {
        let mut response = try!(self.http_client
            .post(&format!("{}/api/v1/datapoints", self.base_url))
            .body(&datapoints.get_json())
            .send());
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
    fn get_version_unknown_host() {
        let client = Client::new("unknown", 8080);
        assert!(client.version().unwrap().starts_with("KairosDB"));
    }

    #[test]
    fn add_datapoints() {
        let client = Client::new("unknown", 8080);
        let datapoints = Datapoints::new("first", 0);
        client.add(&datapoints);
    }
}
