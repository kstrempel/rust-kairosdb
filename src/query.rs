// Copyright 2016-2020 Kai Strempel
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Several structs to create and parse queries

use std::collections::HashMap;
use chrono::{DateTime, Local, Utc};

/// Internal tag type
pub type Tags = HashMap<String, Vec<String>>;

/// Enum for different time units
#[derive(Serialize, Deserialize, Debug)]
pub enum TimeUnit {
    MILLISECONDS,
    SECONDS,
    MINUTES,
    HOURS,
    DAYS,
    WEEKS,
    MONTHS,
    YEARS,
}

/// Aggregator methods
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
    HISTOGRAM,
}

/// JSON representation of a kairosdb query
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

    metrics: Vec<Metric>,
}

/// JSON representation of the a relative time object
#[derive(Serialize, Deserialize, Debug)]
pub struct RelativeTime {
    value: i64,
    unit: TimeUnit,
}

/// Type to support the several time definitions for the client
pub enum Time {
    UTC(DateTime<Utc>),
    Local(DateTime<Local>),
    Nanoseconds(i64),
    Relative { value: i64, unit: TimeUnit },
}

/// JSON representation of the metric object
#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    tags: Tags,
    name: String,
    aggregators: Vec<Aggregator>,
}

/// JSON representation of the aggregator object
#[derive(Serialize, Deserialize, Debug)]
pub struct Aggregator {
    name: AggregatorType,
    sampling: RelativeTime,
}

/// JSON representation of the sampling object
#[derive(Serialize, Deserialize, Debug)]
pub struct Sampling {
    value: i64,
    unit: TimeUnit,
}

impl Query {
    /// Creates a new `Query` object. With a absolute or relative
    /// start and end time.
    ///
    /// The following example creates a query starting from the
    /// first nanosecond in our world until the first week in our world
    /// finished.
    /// ```
    /// # use kairosdb::query::{Query, Time, Metric, Tags, TimeUnit};
    /// let query = Query::new(Time::Nanoseconds(1),
    ///    Time::Relative{value: 1, unit: TimeUnit::WEEKS});
    /// ```
    pub fn new(start: Time, end: Time) -> Query {
        Query {
            start_absolute: match start {
                Time::Nanoseconds(n) => Some(n),
                Time::Local(n) => Some(n.timestamp() * 1000),
                Time::UTC(n) => Some(n.timestamp() * 1000),
                _ => None,
            },
            end_absolute: match end {
                Time::Nanoseconds(n) => Some(n),
                Time::Local(n) => Some(n.timestamp() * 1000),
                Time::UTC(n) => Some(n.timestamp() * 1000),
                _ => None,
            },
            start_relative: match start {
                Time::Relative { value, unit } => {
                    Some(RelativeTime {
                        value,
                        unit
                    })
                }
                _ => None,
            },
            end_relative: match end {
                Time::Relative { value, unit } => {
                    Some(RelativeTime {
                        value,
                        unit
                    })
                }
                _ => None,
            },
            metrics: vec![],
        }
    }

    pub fn add(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }
}

impl Metric {
    /// Creates  a new `Metric` object.
    pub fn new(name: &str, tags: Tags, aggregators: Vec<Aggregator>) -> Metric {
        Metric {
            tags,
            name: name.to_string(),
            aggregators
        }
    }
}

impl Aggregator {
    /// Creates a new `Aggregator` object
    pub fn new(name: AggregatorType, sampling: RelativeTime) -> Aggregator {
        Aggregator {
            name,
            sampling
        }
    }
}

impl RelativeTime {
    /// Creates a new `RelativeTime` object
    pub fn new(value: i64, unit: TimeUnit) -> RelativeTime {
        RelativeTime {
            value,
            unit
        }
    }
}
