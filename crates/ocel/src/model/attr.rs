use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Declared attribute data type (the `type` field on event/object type attributes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttrType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "time")]
    Time,
}

/// A typed attribute value.
///
/// Serialized untagged so it maps onto the natural JSON value shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttrValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Time(DateTime<Utc>),
    String(String),
}
