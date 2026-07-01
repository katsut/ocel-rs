//! Format readers and writers for OCEL 2.0.

pub mod json;
pub mod sqlite;

use thiserror::Error;

/// Errors from reading or writing OCEL logs.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("format error: {0}")]
    Format(String),
}
