// Copyright 2016-2018 Kai Strempel
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

extern crate serde_json;
use std::collections::HashMap;

use error::KairoError;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    queries: Vec<Query>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    sample_size: i64,
    results: Vec<ResultValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultValues {
    name: String,
    values: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct Value {
    pub time: u64,
    pub value: f64,
}

pub type ResultMap = HashMap<String, ResultVector>;
type ResultVector = Vec<Value>;

impl QueryResult {
    pub fn new() -> QueryResult {
        QueryResult { queries: vec![] }
    }

    pub fn parse_result(&self, body: &str) -> Result<ResultMap, KairoError> {
        let mut result: ResultMap = HashMap::new();
        let deserialized: QueryResult = serde_json::from_str(body)?;

        for query in deserialized.queries {
            for r in query.results {
                let mut values: ResultVector = Vec::new();
                for v in r.values {
                    values.push(Value {
                        time: v[0] as u64,
                        value: v[1] as f64,
                    });
                }
                result.insert(r.name, values);
            }
        }

        Ok(result)
    }
}
