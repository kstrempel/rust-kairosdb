use std::collections::HashMap; 

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeUnit {
    MILLISECONDS,
    SECONDS,
    MINUTES,
    HOURS,
    DAYS,
    WEEKS,
    MONTHS,
    YEARS
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    metrics: Vec<Metrics>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    tags: HashMap<String, Vec<String>>,
    name: String,
    limit: i64,
    aggregators: Vec<Aggregator>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aggregator {
    name: String,
    samling: Sampling
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sampling {
    value: i64,
    unit: TimeUnit
}

impl Query {
    pub fn new() -> Query {
        Query{
            metrics: vec![]
        }
    }
}
