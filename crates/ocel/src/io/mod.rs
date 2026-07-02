//! Format readers and writers for OCEL 2.0.

pub(crate) mod coerce;
pub mod json;
pub mod sqlite;
pub mod xml;

use std::path::Path;

use thiserror::Error;

use crate::model::Ocel;

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

fn format_of(path: &Path) -> Result<Format, IoError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "json" | "jsonocel" => Ok(Format::Json),
        "sqlite" | "db" => Ok(Format::Sqlite),
        "xml" | "xmlocel" => Ok(Format::Xml),
        other => Err(IoError::Format(format!(
            "unknown file extension: {other:?}"
        ))),
    }
}

#[derive(Debug, Clone, Copy)]
enum Format {
    Json,
    Sqlite,
    Xml,
}

/// Read an [`Ocel`], choosing the format by file extension
/// (`.json`/`.jsonocel`, `.sqlite`/`.db`, `.xml`/`.xmlocel`).
pub fn read_path<P: AsRef<Path>>(path: P) -> Result<Ocel, IoError> {
    let path = path.as_ref();
    match format_of(path)? {
        Format::Json => json::read_path(path),
        Format::Sqlite => sqlite::read_path(path),
        Format::Xml => xml::read_path(path),
    }
}

/// Write an [`Ocel`], choosing the format by file extension
/// (`.json`/`.jsonocel`, `.sqlite`/`.db`, `.xml`/`.xmlocel`).
pub fn write_path<P: AsRef<Path>>(ocel: &Ocel, path: P) -> Result<(), IoError> {
    let path = path.as_ref();
    match format_of(path)? {
        Format::Json => json::write_path(ocel, path),
        Format::Sqlite => sqlite::write_path(ocel, path),
        Format::Xml => xml::write_path(ocel, path),
    }
}
