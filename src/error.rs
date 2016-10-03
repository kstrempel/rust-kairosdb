use hyper;
use serde_json;
use std;

#[derive(Debug)]
pub enum KairoError {
    KairoError(String),
    HttpError(hyper::error::Error),
    JsonError(serde_json::error::Error),
    IOError(std::io::Error),
}

impl From<hyper::error::Error> for KairoError {
    fn from(err: hyper::error::Error) -> KairoError {
        KairoError::HttpError(err)
    }
}

impl From<serde_json::error::Error> for KairoError {
    fn from(err: serde_json::error::Error) -> KairoError {
        KairoError::JsonError(err)
    }
}

impl From<std::io::Error> for KairoError {
    fn from(err: std::io::Error) -> KairoError {
        KairoError::IOError(err)
    }
}
