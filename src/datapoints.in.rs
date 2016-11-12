use std::collections::HashMap;
use chrono::{DateTime, TimeZone};

#[derive(Serialize, Deserialize, Debug)]
pub struct Datapoints {
    name: String,
    datapoints: Vec<(i64, f64)>,
    tags: HashMap<String,String>,
    ttl: u32
}

impl Datapoints {
    pub fn new(name: &str, ttl: u32) -> Datapoints {
        Datapoints{
            name: name.to_string(),
            datapoints: Vec::new(),
            tags: HashMap::new(),
            ttl: ttl
        }
    }

    pub fn add<Tz: TimeZone>(&mut self , datetime: DateTime<Tz>, value: f64) {
        self.datapoints.push((datetime.timestamp() * 1000, value));
    }

    pub fn add_ms(&mut self , ns: i64, value: f64) {
        self.datapoints.push((ns, value));
    }

    pub fn add_tag(&mut self, name: &str, value: &str) {
        self.tags.insert(name.to_string(), value.to_string());
    }
}
