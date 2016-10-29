extern crate serde_json;
use std::collections::HashMap;

use error::KairoError;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    queries: Vec<Query>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    sample_size: i64,
    results : Vec<ResultValues>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultValues {
    name: String,
    values: Vec<Vec<i64>>
}

#[derive(Debug)]
pub struct Value {
    pub time: u64,
    pub value: i64
}

pub type ResultMap = HashMap<String, ResultVector>;
type ResultVector = Vec<Value>;

impl QueryResult {
    pub fn new() -> QueryResult {
        QueryResult{queries: vec![]}
    }

    pub fn parse_result(&self, body: &String) -> Result<ResultMap,
                                                        KairoError> {
        let mut result : ResultMap = HashMap::new();
        let deserialized : QueryResult = try!(serde_json::from_str(body));

        for query in deserialized.queries {
            for r in query.results {
                let mut values : ResultVector = Vec::new();
                for v in r.values {
                    values.push(Value{time: v[0] as u64, value: v[1]});
                }
                result.insert(r.name, values);
            }
        }

        Ok(result)
    }
}
