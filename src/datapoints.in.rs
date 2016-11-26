/*
 * Copyright 2016 Kai Strempel
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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

    pub fn add_ms(&mut self , ms: i64, value: f64) {
        self.datapoints.push((ms, value));
    }

    pub fn add_tag(&mut self, name: &str, value: &str) {
        self.tags.insert(name.to_string(), value.to_string());
    }
}
