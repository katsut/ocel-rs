//! Format readers and writers for OCEL 2.0.

pub mod json;

use thiserror::Error;

/// Errors from reading or writing OCEL logs.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
