//! OCEL 2.0 semantic validation.
//!
//! Checks an in-memory [`Ocel`] against the OCEL 2.0 specification and reports
//! every violation found (not just the first). Format readers may accept lenient
//! input; this is where spec conformance is enforced.

use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::model::{AttributeDefinition, Ocel};

/// A single OCEL 2.0 conformance violation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Violation {
    #[error("duplicate event id: {0}")]
    DuplicateEventId(String),

    #[error("duplicate object id: {0}")]
    DuplicateObjectId(String),

    #[error("event {event} has undeclared event type {event_type}")]
    UndeclaredEventType { event: String, event_type: String },

    #[error("object {object} has undeclared object type {object_type}")]
    UndeclaredObjectType { object: String, object_type: String },

    #[error("event {event} has undeclared attribute {attribute}")]
    UndeclaredEventAttribute { event: String, attribute: String },

    #[error("object {object} has undeclared attribute {attribute}")]
    UndeclaredObjectAttribute { object: String, attribute: String },

    #[error("event {event} references unknown object {object}")]
    DanglingE2O { event: String, object: String },

    #[error("object {source_id} references unknown object {target_id}")]
    DanglingO2O {
        source_id: String,
        target_id: String,
    },

    #[error("attribute name {attribute} is declared in more than one type")]
    AttributeNameCollision { attribute: String },
}

impl Ocel {
    /// Validate the log against the OCEL 2.0 specification.
    ///
    /// # Errors
    ///
    /// Returns every [`Violation`] found when the log is not conformant.
    pub fn validate(&self) -> Result<(), Vec<Violation>> {
        let violations = validate(self);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Collect all OCEL 2.0 conformance violations in `ocel`.
#[must_use]
pub fn validate(ocel: &Ocel) -> Vec<Violation> {
    let mut violations = Vec::new();

    let mut object_ids: HashSet<&str> = HashSet::with_capacity(ocel.objects.len());
    for object in &ocel.objects {
        if !object_ids.insert(object.id.as_str()) {
            violations.push(Violation::DuplicateObjectId(object.id.clone()));
        }
    }
    let mut event_ids: HashSet<&str> = HashSet::with_capacity(ocel.events.len());
    for event in &ocel.events {
        if !event_ids.insert(event.id.as_str()) {
            violations.push(Violation::DuplicateEventId(event.id.clone()));
        }
    }

    let event_type_attrs = type_attr_map(&ocel.event_types);
    let object_type_attrs = type_attr_map(&ocel.object_types);
    check_attr_disjoint(&ocel.event_types, &mut violations);
    check_attr_disjoint(&ocel.object_types, &mut violations);

    for event in &ocel.events {
        if let Some(declared) = event_type_attrs.get(event.event_type.as_str()) {
            for attr in &event.attributes {
                if !declared.contains(attr.name.as_str()) {
                    violations.push(Violation::UndeclaredEventAttribute {
                        event: event.id.clone(),
                        attribute: attr.name.clone(),
                    });
                }
            }
        } else {
            violations.push(Violation::UndeclaredEventType {
                event: event.id.clone(),
                event_type: event.event_type.clone(),
            });
        }
        for rel in &event.relationships {
            if !object_ids.contains(rel.object_id.as_str()) {
                violations.push(Violation::DanglingE2O {
                    event: event.id.clone(),
                    object: rel.object_id.clone(),
                });
            }
        }
    }

    for object in &ocel.objects {
        if let Some(declared) = object_type_attrs.get(object.object_type.as_str()) {
            for attr in &object.attributes {
                if !declared.contains(attr.name.as_str()) {
                    violations.push(Violation::UndeclaredObjectAttribute {
                        object: object.id.clone(),
                        attribute: attr.name.clone(),
                    });
                }
            }
        } else {
            violations.push(Violation::UndeclaredObjectType {
                object: object.id.clone(),
                object_type: object.object_type.clone(),
            });
        }
        for rel in &object.relationships {
            if !object_ids.contains(rel.object_id.as_str()) {
                violations.push(Violation::DanglingO2O {
                    source_id: object.id.clone(),
                    target_id: rel.object_id.clone(),
                });
            }
        }
    }

    violations
}

trait TypeDecl {
    fn type_name(&self) -> &str;
    fn declared_attributes(&self) -> &[AttributeDefinition];
}

impl TypeDecl for crate::model::EventType {
    fn type_name(&self) -> &str {
        &self.name
    }
    fn declared_attributes(&self) -> &[AttributeDefinition] {
        &self.attributes
    }
}

impl TypeDecl for crate::model::ObjectType {
    fn type_name(&self) -> &str {
        &self.name
    }
    fn declared_attributes(&self) -> &[AttributeDefinition] {
        &self.attributes
    }
}

fn type_attr_map<T: TypeDecl>(types: &[T]) -> HashMap<&str, HashSet<&str>> {
    types
        .iter()
        .map(|t| {
            let attrs = t
                .declared_attributes()
                .iter()
                .map(|a| a.name.as_str())
                .collect();
            (t.type_name(), attrs)
        })
        .collect()
}

fn check_attr_disjoint<T: TypeDecl>(types: &[T], violations: &mut Vec<Violation>) {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut reported: HashSet<&str> = HashSet::new();
    for t in types {
        for attr in t.declared_attributes() {
            let name = attr.name.as_str();
            if !seen.insert(name) && reported.insert(name) {
                violations.push(Violation::AttributeNameCollision {
                    attribute: name.to_owned(),
                });
            }
        }
    }
}
