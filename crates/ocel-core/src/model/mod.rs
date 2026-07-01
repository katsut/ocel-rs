//! OCEL 2.0 data model (`OCEL`-native, per ADR 0001).

mod attr;
mod builder;

pub use attr::{AttrType, AttrValue};
pub use builder::OcelBuilder;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An attribute definition on an event type or object type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: AttrType,
}

/// An event type declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventType {
    pub name: String,
    #[serde(default)]
    pub attributes: Vec<AttributeDefinition>,
}

/// An object type declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectType {
    pub name: String,
    #[serde(default)]
    pub attributes: Vec<AttributeDefinition>,
}

/// A static event attribute (no timestamp).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventAttribute {
    pub name: String,
    pub value: AttrValue,
}

/// A dynamic object attribute value at a point in time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectAttribute {
    pub name: String,
    pub value: AttrValue,
    pub time: DateTime<Utc>,
}

/// A qualified relationship reference: `E2O` when on an [`Event`], `O2O` when on an [`Object`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "objectId")]
    pub object_id: String,
    pub qualifier: String,
}

/// An event: a timestamped activity execution with `E2O` relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub attributes: Vec<EventAttribute>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

impl Object {
    /// The value of dynamic attribute `name` at time `t`, using forward-fill
    /// (the latest value recorded at or before `t`).
    #[must_use]
    pub fn attribute_at(&self, name: &str, t: DateTime<Utc>) -> Option<&AttrValue> {
        self.attributes
            .iter()
            .filter(|a| a.name == name && a.time <= t)
            .max_by_key(|a| a.time)
            .map(|a| &a.value)
    }
}

/// An object: a process entity with dynamic attributes and `O2O` relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    #[serde(default)]
    pub attributes: Vec<ObjectAttribute>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

/// A flattened event-to-object (`E2O`) relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E2ORelation<'a> {
    pub event_id: &'a str,
    pub object_id: &'a str,
    pub qualifier: &'a str,
}

/// A flattened object-to-object (`O2O`) relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct O2ORelation<'a> {
    pub source_id: &'a str,
    pub target_id: &'a str,
    pub qualifier: &'a str,
}

/// A columnar view of events (entry point for `Arrow` / `Polars` / `PyO3` bindings).
#[derive(Debug, Clone, PartialEq)]
pub struct EventColumns<'a> {
    pub ids: Vec<&'a str>,
    pub types: Vec<&'a str>,
    pub times: Vec<DateTime<Utc>>,
}

/// An OCEL 2.0 event log.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Ocel {
    #[serde(rename = "eventTypes", default)]
    pub event_types: Vec<EventType>,
    #[serde(rename = "objectTypes", default)]
    pub object_types: Vec<ObjectType>,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub objects: Vec<Object>,
}

impl Ocel {
    /// Start a new [`OcelBuilder`].
    #[must_use]
    pub fn builder() -> OcelBuilder {
        OcelBuilder::new()
    }

    /// Iterate flattened `E2O` relations across all events.
    pub fn e2o(&self) -> impl Iterator<Item = E2ORelation<'_>> + '_ {
        self.events.iter().flat_map(|e| {
            e.relationships.iter().map(move |r| E2ORelation {
                event_id: &e.id,
                object_id: &r.object_id,
                qualifier: &r.qualifier,
            })
        })
    }

    /// Iterate flattened `O2O` relations across all objects.
    pub fn o2o(&self) -> impl Iterator<Item = O2ORelation<'_>> + '_ {
        self.objects.iter().flat_map(|o| {
            o.relationships.iter().map(move |r| O2ORelation {
                source_id: &o.id,
                target_id: &r.object_id,
                qualifier: &r.qualifier,
            })
        })
    }

    /// A columnar view of the events.
    #[must_use]
    pub fn event_columns(&self) -> EventColumns<'_> {
        EventColumns {
            ids: self.events.iter().map(|e| e.id.as_str()).collect(),
            types: self.events.iter().map(|e| e.event_type.as_str()).collect(),
            times: self.events.iter().map(|e| e.time).collect(),
        }
    }
}
