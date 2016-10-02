#[cfg(feature = "serde_macros")]
include!("datapoints.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/datapoints.rs"));
