extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;

pub mod datapoints;
pub mod query;
pub mod result;
mod error;

#[cfg(feature = "serde_macros")]
include!("lib.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
