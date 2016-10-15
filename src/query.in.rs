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
    metrics: Vec<Metric>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
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

    pub fn add(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }
}

impl Metric {
    pub fn new(name: &str,
               tags: HashMap<String, Vec<String>>,
               aggregators: Vec<Aggregator>) -> Metric {
        Metric{
            tags: tags,
            name: name.to_string(),
            limit: 10,
            aggregators: aggregators
        }
    }
}
