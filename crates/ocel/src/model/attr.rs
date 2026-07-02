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

impl AttrValue {
    /// The [`AttrType`] of this value.
    #[must_use]
    pub fn attr_type(&self) -> AttrType {
        match self {
            AttrValue::String(_) => AttrType::String,
            AttrValue::Integer(_) => AttrType::Integer,
            AttrValue::Float(_) => AttrType::Float,
            AttrValue::Boolean(_) => AttrType::Boolean,
            AttrValue::Time(_) => AttrType::Time,
        }
    }

    /// The canonical text form used by the text-based formats
    /// (times as RFC 3339).
    #[must_use]
    pub fn to_text(&self) -> String {
        match self {
            AttrValue::String(s) => s.clone(),
            AttrValue::Integer(i) => i.to_string(),
            AttrValue::Float(f) => f.to_string(),
            AttrValue::Boolean(b) => b.to_string(),
            AttrValue::Time(t) => t.to_rfc3339(),
        }
    }
}
