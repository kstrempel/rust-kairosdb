extern crate serde_json;

use error::KairoError;


#[derive(Serialize, Deserialize, Debug)]
struct Metricnames {
    results: Vec<String>
}

pub fn parse_metricnames_result(body: &str) -> Result<Vec<String>,
                                                      KairoError> {
    let deserialized : Metricnames = try!(serde_json::from_str(body));
    Ok(deserialized.results)
}
