
pub struct Datapoints {
    name: String,
    datapoints: String,
    tags: String,
    ttl: u32
}

impl Datapoints {
    pub fn new(name: &str, ttl: u32) -> Datapoints {
        Datapoints{
            name: name.to_string(),
            datapoints: "".to_string(),
            tags: "".to_string(),
            ttl: ttl
        }
    }

    pub fn get_json(&self) -> String {
        "".to_string()
    }
}
