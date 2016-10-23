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
    #[serde(skip_serializing_if = "Option::is_none")]
    start_absolute: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_absolute: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_relative: Option<RelativeTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_relative: Option<RelativeTime>,

    metrics: Vec<Metric>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelativeTime {
    value: i64,
    unit: TimeUnit
}

pub enum Time {
    Absolute(i64),
    Relative{value: i64, unit: TimeUnit}
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
    sampling: RelativeTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sampling {
    value: i64,
    unit: TimeUnit
}

impl Query {
    pub fn new(start: Time, end: Time) -> Query {
        Query{
            start_absolute: match start {
                Time::Absolute(n) => Some(n),
                _ => None
            },
            end_absolute: match end {
                Time::Absolute(n) => Some(n),
                _=> None
            },
            start_relative: match start {
                Time::Relative{value, unit} => Some(RelativeTime{
                    value: value,
                    unit: unit}),
                _=> None
            },
            end_relative: match end{
                Time::Relative{value, unit} => Some(RelativeTime{
                    value: value,
                    unit: unit}),
                _=> None
            },
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
