//! Declaration-driven attribute typing shared by the format readers.
//!
//! Formats carry attribute values as text (`XML`, official-style JSON) or with
//! partial typing (`SQLite` column affinity, native JSON values). After parsing,
//! [`apply_declared_types`] coerces every attribute value to the type declared on
//! its event/object type, so the resulting [`Ocel`] is identical no matter which
//! format it was read from. Unconvertible values are left as-is (lenient read).

use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::model::{AttrType, AttrValue, AttributeDefinition, Ocel};

/// Parse a timestamp accepting RFC 3339 as well as offset-less ISO 8601 /
/// space-separated forms (treated as UTC).
pub(crate) fn parse_time_lenient(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.to_utc());
    }
    for fmt in [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
    ] {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(ndt.and_utc());
        }
    }
    None
}

fn attr_map(attributes: &[AttributeDefinition]) -> HashMap<&str, AttrType> {
    attributes
        .iter()
        .map(|a| (a.name.as_str(), a.value_type))
        .collect()
}

/// Coerce every attribute value in `ocel` to its declared type.
pub(crate) fn apply_declared_types(ocel: &mut Ocel) {
    let event_types: HashMap<String, HashMap<&str, AttrType>> = ocel
        .event_types
        .iter()
        .map(|t| (t.name.clone(), attr_map(&t.attributes)))
        .collect();
    let object_types: HashMap<String, HashMap<&str, AttrType>> = ocel
        .object_types
        .iter()
        .map(|t| (t.name.clone(), attr_map(&t.attributes)))
        .collect();

    let events = std::mem::take(&mut ocel.events);
    ocel.events = events
        .into_iter()
        .map(|mut event| {
            if let Some(declared) = event_types.get(&event.event_type) {
                for attr in &mut event.attributes {
                    if let Some(&ty) = declared.get(attr.name.as_str()) {
                        coerce_value(&mut attr.value, ty);
                    }
                }
            }
            event
        })
        .collect();

    let objects = std::mem::take(&mut ocel.objects);
    ocel.objects = objects
        .into_iter()
        .map(|mut object| {
            if let Some(declared) = object_types.get(&object.object_type) {
                for attr in &mut object.attributes {
                    if let Some(&ty) = declared.get(attr.name.as_str()) {
                        coerce_value(&mut attr.value, ty);
                    }
                }
            }
            object
        })
        .collect();
}

#[allow(clippy::cast_precision_loss)]
fn coerce_value(value: &mut AttrValue, target: AttrType) {
    let coerced = match (target, &*value) {
        (AttrType::String, AttrValue::String(_)) => None,
        (AttrType::String, v) => Some(AttrValue::String(v.to_text())),
        (AttrType::Integer, AttrValue::String(s)) => {
            s.trim().parse::<i64>().ok().map(AttrValue::Integer)
        }
        (AttrType::Float, AttrValue::String(s)) => {
            s.trim().parse::<f64>().ok().map(AttrValue::Float)
        }
        (AttrType::Float, AttrValue::Integer(i)) => Some(AttrValue::Float(*i as f64)),
        (AttrType::Boolean, AttrValue::String(s)) => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" => Some(AttrValue::Boolean(true)),
            "false" | "0" => Some(AttrValue::Boolean(false)),
            _ => None,
        },
        (AttrType::Boolean, AttrValue::Integer(i)) => Some(AttrValue::Boolean(*i != 0)),
        (AttrType::Time, AttrValue::String(s)) => parse_time_lenient(s).map(AttrValue::Time),
        _ => None,
    };
    if let Some(v) = coerced {
        *value = v;
    }
}
