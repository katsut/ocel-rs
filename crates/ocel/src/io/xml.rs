//! OCEL 2.0 `XML` reader and writer.
//!
//! Maps the OCEL 2.0 XML shape (`<log>` with `object-types` / `event-types` /
//! `objects` / `events`) to and from the model. Attribute values are text and are
//! coerced to their declared types after parsing; timestamps accept both
//! `Z`-suffixed and offset-less ISO 8601 (treated as UTC).

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::coerce::{apply_declared_types, parse_time_lenient};
use crate::io::IoError;
use crate::model::{
    AttrType, AttrValue, AttributeDefinition, Event, EventAttribute, EventType, Object,
    ObjectAttribute, ObjectType, Ocel, Relationship,
};

const XML_DECL: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";

// ---------------------------------------------------------------------------
// serde DTOs (quick-xml shape)
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "log")]
struct XmlLog {
    #[serde(rename = "object-types", default)]
    object_types: ObjectTypes,
    #[serde(rename = "event-types", default)]
    event_types: EventTypes,
    #[serde(default)]
    objects: Objects,
    #[serde(default)]
    events: Events,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ObjectTypes {
    #[serde(rename = "object-type", default)]
    items: Vec<TypeDecl>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct EventTypes {
    #[serde(rename = "event-type", default)]
    items: Vec<TypeDecl>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeDecl {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default)]
    attributes: AttrDecls,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct AttrDecls {
    #[serde(rename = "attribute", default)]
    items: Vec<AttrDecl>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttrDecl {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    ty: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Objects {
    #[serde(rename = "object", default)]
    items: Vec<ObjectX>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectX {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    ty: String,
    #[serde(default)]
    attributes: ObjectAttrs,
    #[serde(default)]
    objects: Option<Relationships>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ObjectAttrs {
    #[serde(rename = "attribute", default)]
    items: Vec<ObjectAttrX>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectAttrX {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@time")]
    time: String,
    #[serde(rename = "$text", default)]
    value: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Events {
    #[serde(rename = "event", default)]
    items: Vec<EventX>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventX {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    ty: String,
    #[serde(rename = "@time")]
    time: String,
    #[serde(default)]
    attributes: EventAttrs,
    #[serde(default)]
    objects: Option<Relationships>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct EventAttrs {
    #[serde(rename = "attribute", default)]
    items: Vec<EventAttrX>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventAttrX {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text", default)]
    value: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Relationships {
    #[serde(rename = "relationship", default)]
    items: Vec<RelationshipX>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipX {
    #[serde(rename = "@object-id")]
    object_id: String,
    #[serde(rename = "@qualifier")]
    qualifier: String,
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn attr_type_from_str(s: &str) -> AttrType {
    match s {
        "integer" => AttrType::Integer,
        "float" => AttrType::Float,
        "boolean" => AttrType::Boolean,
        "time" => AttrType::Time,
        _ => AttrType::String,
    }
}

fn attr_type_to_str(ty: AttrType) -> &'static str {
    match ty {
        AttrType::String => "string",
        AttrType::Integer => "integer",
        AttrType::Float => "float",
        AttrType::Boolean => "boolean",
        AttrType::Time => "time",
    }
}

fn parse_time(s: &str) -> Result<DateTime<Utc>, IoError> {
    parse_time_lenient(s).ok_or_else(|| IoError::Format(format!("invalid timestamp: {s}")))
}

// ---------------------------------------------------------------------------
// conversion
// ---------------------------------------------------------------------------

fn type_decls(items: Vec<TypeDecl>) -> Vec<(String, Vec<AttributeDefinition>)> {
    items
        .into_iter()
        .map(|t| {
            let attrs = t
                .attributes
                .items
                .into_iter()
                .map(|a| AttributeDefinition {
                    name: a.name,
                    value_type: attr_type_from_str(&a.ty),
                })
                .collect();
            (t.name, attrs)
        })
        .collect()
}

fn relationships(rels: Option<Relationships>) -> Vec<Relationship> {
    rels.map(|r| r.items)
        .unwrap_or_default()
        .into_iter()
        .map(|r| Relationship {
            object_id: r.object_id,
            qualifier: r.qualifier,
        })
        .collect()
}

fn to_model(log: XmlLog) -> Result<Ocel, IoError> {
    let event_types = type_decls(log.event_types.items)
        .into_iter()
        .map(|(name, attributes)| EventType { name, attributes })
        .collect();
    let object_types = type_decls(log.object_types.items)
        .into_iter()
        .map(|(name, attributes)| ObjectType { name, attributes })
        .collect();

    let mut events = Vec::with_capacity(log.events.items.len());
    for e in log.events.items {
        let attributes = e
            .attributes
            .items
            .into_iter()
            .map(|a| EventAttribute {
                name: a.name,
                value: AttrValue::String(a.value),
            })
            .collect();
        events.push(Event {
            id: e.id,
            event_type: e.ty,
            time: parse_time(&e.time)?,
            attributes,
            relationships: relationships(e.objects),
        });
    }

    let mut objects = Vec::with_capacity(log.objects.items.len());
    for o in log.objects.items {
        let mut attributes = Vec::with_capacity(o.attributes.items.len());
        for a in o.attributes.items {
            attributes.push(ObjectAttribute {
                name: a.name,
                value: AttrValue::String(a.value),
                time: parse_time(&a.time)?,
            });
        }
        objects.push(Object {
            id: o.id,
            object_type: o.ty,
            attributes,
            relationships: relationships(o.objects),
        });
    }

    let mut ocel = Ocel {
        event_types,
        object_types,
        events,
        objects,
    };
    apply_declared_types(&mut ocel);
    Ok(ocel)
}

fn rel_dtos(rels: &[Relationship]) -> Option<Relationships> {
    if rels.is_empty() {
        return None;
    }
    Some(Relationships {
        items: rels
            .iter()
            .map(|r| RelationshipX {
                object_id: r.object_id.clone(),
                qualifier: r.qualifier.clone(),
            })
            .collect(),
    })
}

fn from_model(ocel: &Ocel) -> XmlLog {
    let object_types = ObjectTypes {
        items: ocel
            .object_types
            .iter()
            .map(|t| type_decl_dto(&t.name, &t.attributes))
            .collect(),
    };
    let event_types = EventTypes {
        items: ocel
            .event_types
            .iter()
            .map(|t| type_decl_dto(&t.name, &t.attributes))
            .collect(),
    };
    let objects = Objects {
        items: ocel
            .objects
            .iter()
            .map(|o| ObjectX {
                id: o.id.clone(),
                ty: o.object_type.clone(),
                attributes: ObjectAttrs {
                    items: o
                        .attributes
                        .iter()
                        .map(|a| ObjectAttrX {
                            name: a.name.clone(),
                            time: a.time.to_rfc3339(),
                            value: a.value.to_text(),
                        })
                        .collect(),
                },
                objects: rel_dtos(&o.relationships),
            })
            .collect(),
    };
    let events = Events {
        items: ocel
            .events
            .iter()
            .map(|e| EventX {
                id: e.id.clone(),
                ty: e.event_type.clone(),
                time: e.time.to_rfc3339(),
                attributes: EventAttrs {
                    items: e
                        .attributes
                        .iter()
                        .map(|a| EventAttrX {
                            name: a.name.clone(),
                            value: a.value.to_text(),
                        })
                        .collect(),
                },
                objects: rel_dtos(&e.relationships),
            })
            .collect(),
    };
    XmlLog {
        object_types,
        event_types,
        objects,
        events,
    }
}

fn type_decl_dto(name: &str, attributes: &[AttributeDefinition]) -> TypeDecl {
    TypeDecl {
        name: name.to_owned(),
        attributes: AttrDecls {
            items: attributes
                .iter()
                .map(|a| AttrDecl {
                    name: a.name.clone(),
                    ty: attr_type_to_str(a.value_type).to_owned(),
                })
                .collect(),
        },
    }
}

// ---------------------------------------------------------------------------
// public API
// ---------------------------------------------------------------------------

/// Parse an [`Ocel`] from an OCEL 2.0 `XML` string.
pub fn read_str(s: &str) -> Result<Ocel, IoError> {
    let log: XmlLog = quick_xml::de::from_str(s).map_err(|e| IoError::Xml(e.to_string()))?;
    to_model(log)
}

/// Read an [`Ocel`] from an OCEL 2.0 `XML` file.
pub fn read_path<P: AsRef<Path>>(path: P) -> Result<Ocel, IoError> {
    let text = std::fs::read_to_string(path)?;
    read_str(&text)
}

/// Serialize an [`Ocel`] to an OCEL 2.0 `XML` string.
pub fn write_string(ocel: &Ocel) -> Result<String, IoError> {
    let log = from_model(ocel);
    let body = quick_xml::se::to_string(&log).map_err(|e| IoError::Xml(e.to_string()))?;
    Ok(format!("{XML_DECL}{body}"))
}

/// Write an [`Ocel`] as OCEL 2.0 `XML` to a file path.
pub fn write_path<P: AsRef<Path>>(ocel: &Ocel, path: P) -> Result<(), IoError> {
    std::fs::write(path, write_string(ocel)?)?;
    Ok(())
}
