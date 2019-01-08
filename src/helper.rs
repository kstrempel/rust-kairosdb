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

use error::KairoError;


#[derive(Serialize, Deserialize, Debug)]
struct Metricnames {
    results: Vec<String>,
}

pub fn parse_metricnames_result(body: &str) -> Result<Vec<String>, KairoError> {
    let deserialized: Metricnames = try!(serde_json::from_str(body));
    Ok(deserialized.results)
}
