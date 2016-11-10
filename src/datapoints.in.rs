use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Datapoints {
    name: String,
    datapoints: Vec<Vec<f64>>,
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

    pub fn add(&mut self , ms: i64, value: f64) {
        self.datapoints.push(vec![ms as f64, value]);
    }

    pub fn add_tag(&mut self, name: &str, value: &str) {
        self.tags.insert(name.to_string(), value.to_string());
    }
}
