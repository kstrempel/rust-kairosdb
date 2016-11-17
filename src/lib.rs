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

#![crate_type = "lib"]
#![crate_name = "kairosdb"]

//!
//! A `Client` for KairosBD REST API
//!
//! The Client itself is used as the central access point, from which
//! numerous operations are defined implementing each of the specific
//! KairosDB APIs.

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate chrono;

pub mod datapoints;
pub mod query;
pub mod result;
mod error;

#[cfg(feature = "serde_macros")]
include!("lib.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
