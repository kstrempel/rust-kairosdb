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
    fn from(err: hyper::error::Error) -> KairoError{
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
