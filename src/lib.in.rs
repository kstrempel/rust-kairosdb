use std::io::Read;

pub struct Client {
    base_url:    String,
    http_client: hyper::Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct Version{
    version: String
}

impl Client {
    pub fn new(host: &str, port: u32) -> Client {
        Client{
            base_url: format!("http://{}:{}", host, port),
            http_client: hyper::Client::new()
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
}


#[cfg(test)]
mod tests {

    use super::Client;

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

}
