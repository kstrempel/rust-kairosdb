extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate hyper;

mod datapoints;

#[cfg(feature = "serde_macros")]
include!("lib.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
