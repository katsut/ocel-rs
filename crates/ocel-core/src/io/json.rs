//! OCEL 2.0 JSON reader and writer.
//!
//! The in-memory model derives `serde` in the OCEL 2.0 JSON shape, so reading and
//! writing are thin wrappers over `serde_json`.

use std::io::{Read, Write};
use std::path::Path;

use crate::io::IoError;
use crate::model::Ocel;

/// Parse an [`Ocel`] from a JSON string.
pub fn read_str(s: &str) -> Result<Ocel, IoError> {
    serde_json::from_str(s).map_err(IoError::from)
}

/// Read an [`Ocel`] from a reader.
pub fn read_reader<R: Read>(reader: R) -> Result<Ocel, IoError> {
    serde_json::from_reader(reader).map_err(IoError::from)
}

/// Read an [`Ocel`] from a file path.
pub fn read_path<P: AsRef<Path>>(path: P) -> Result<Ocel, IoError> {
    let file = std::fs::File::open(path)?;
    read_reader(std::io::BufReader::new(file))
}

/// Serialize an [`Ocel`] to a pretty-printed JSON string.
pub fn write_string(ocel: &Ocel) -> Result<String, IoError> {
    serde_json::to_string_pretty(ocel).map_err(IoError::from)
}

/// Write an [`Ocel`] as pretty-printed JSON to a writer.
pub fn write_writer<W: Write>(ocel: &Ocel, writer: W) -> Result<(), IoError> {
    serde_json::to_writer_pretty(writer, ocel)?;
    Ok(())
}

/// Write an [`Ocel`] as pretty-printed JSON to a file path.
pub fn write_path<P: AsRef<Path>>(ocel: &Ocel, path: P) -> Result<(), IoError> {
    let file = std::fs::File::create(path)?;
    write_writer(ocel, std::io::BufWriter::new(file))
}
