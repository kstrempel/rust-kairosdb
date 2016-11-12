use std::collections::HashMap;
use chrono::{DateTime, UTC, Local};

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
pub enum AggregatorType {
    #[serde(rename = "avg")]
    AVG,
    #[serde(rename = "dev")]
    DEV,
    #[serde(rename = "count")]
    COUNT,
    #[serde(rename = "first")]
    FIRST,
    #[serde(rename = "gaps")]
    GAPS,
    #[serde(rename = "histogram")]
    HISTOGRAM
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
    UTC(DateTime<UTC>),
    Local(DateTime<Local>),
    Nanoseconds(i64),
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
    name: AggregatorType,
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
                Time::Nanoseconds(n) => Some(n),
                Time::Local(n) => Some(n.timestamp() * 1000),
                Time::UTC(n) => Some(n.timestamp() * 1000),
                 _ => None
            },
            end_absolute: match end {
                Time::Nanoseconds(n) => Some(n),
                Time::Local(n) => Some(n.timestamp() * 1000),
                Time::UTC(n) => Some(n.timestamp() * 1000),
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

impl Aggregator {
    pub fn new(name: AggregatorType,
               sampling: RelativeTime) -> Aggregator {
        Aggregator{
            name: name,
            sampling: sampling
        }
    }
}

impl RelativeTime {
    pub fn new(value: i64,
               unit: TimeUnit) -> RelativeTime {
        RelativeTime{
            value: value,
            unit: unit
        }
    }
}
