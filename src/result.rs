#[cfg(feature = "serde_macros")]
include!("result.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/result.rs"));
