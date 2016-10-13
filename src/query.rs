#[cfg(feature = "serde_macros")]
include!("query.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/query.rs"));
