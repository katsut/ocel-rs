//! Format readers and writers for OCEL 2.0.

pub(crate) mod coerce;
pub mod json;
pub mod sqlite;
pub mod xml;

use thiserror::Error;

/// Errors from reading or writing OCEL logs.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("xml error: {0}")]
    Xml(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("format error: {0}")]
    Format(String),
}
