// Copyright 2016 Kai Strempel
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

use hyper;
use serde_json;
use std;

#[derive(Debug)]
pub enum KairoError {
    Kairo(String),
    Http(hyper::error::Error),
    Json(serde_json::error::Error),
    IO(std::io::Error),
}

impl From<hyper::error::Error> for KairoError {
    fn from(err: hyper::error::Error) -> KairoError {
        KairoError::Http(err)
    }
}

impl From<serde_json::error::Error> for KairoError {
    fn from(err: serde_json::error::Error) -> KairoError {
        KairoError::Json(err)
    }
}

impl From<std::io::Error> for KairoError {
    fn from(err: std::io::Error) -> KairoError {
        KairoError::IO(err)
    }
}
