#[cfg(feature = "serde_macros")]
include!("helper.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/helper.rs"));
