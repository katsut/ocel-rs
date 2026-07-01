use std::collections::HashSet;

use crate::error::OcelError;
use crate::model::{Event, EventType, Object, ObjectType, Ocel};

/// A fallible builder that validates structural integrity on [`build`](OcelBuilder::build).
///
/// This is the core-level equivalent of the `StagingLog` → `Ocel` gate described in ADR 0001:
/// the raw parts are accumulated freely and checked once, up front.
#[derive(Debug, Default)]
pub struct OcelBuilder {
    event_types: Vec<EventType>,
    object_types: Vec<ObjectType>,
    events: Vec<Event>,
    objects: Vec<Object>,
}

impl OcelBuilder {
    /// Create an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an event type declaration.
    pub fn add_event_type(&mut self, event_type: EventType) {
        self.event_types.push(event_type);
    }

    /// Add an object type declaration.
    pub fn add_object_type(&mut self, object_type: ObjectType) {
        self.object_types.push(object_type);
    }

    /// Add an event.
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Add an object.
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Validate and produce an [`Ocel`].
    ///
    /// # Errors
    ///
    /// Returns [`OcelError`] on duplicate ids, undeclared types, or dangling
    /// `E2O` / `O2O` references.
    pub fn build(self) -> Result<Ocel, OcelError> {
        let mut object_ids: HashSet<&str> = HashSet::with_capacity(self.objects.len());
        for object in &self.objects {
            if !object_ids.insert(object.id.as_str()) {
                return Err(OcelError::DuplicateObjectId(object.id.clone()));
            }
        }

        let mut event_ids: HashSet<&str> = HashSet::with_capacity(self.events.len());
        for event in &self.events {
            if !event_ids.insert(event.id.as_str()) {
                return Err(OcelError::DuplicateEventId(event.id.clone()));
            }
        }

        let event_type_names: HashSet<&str> =
            self.event_types.iter().map(|t| t.name.as_str()).collect();
        let object_type_names: HashSet<&str> =
            self.object_types.iter().map(|t| t.name.as_str()).collect();

        for event in &self.events {
            if !event_type_names.contains(event.event_type.as_str()) {
                return Err(OcelError::UndeclaredEventType {
                    event: event.id.clone(),
                    event_type: event.event_type.clone(),
                });
            }
            for rel in &event.relationships {
                if !object_ids.contains(rel.object_id.as_str()) {
                    return Err(OcelError::DanglingE2O {
                        event: event.id.clone(),
                        object: rel.object_id.clone(),
                    });
                }
            }
        }

        for object in &self.objects {
            if !object_type_names.contains(object.object_type.as_str()) {
                return Err(OcelError::UndeclaredObjectType {
                    object: object.id.clone(),
                    object_type: object.object_type.clone(),
                });
            }
            for rel in &object.relationships {
                if !object_ids.contains(rel.object_id.as_str()) {
                    return Err(OcelError::DanglingO2O {
                        source_id: object.id.clone(),
                        target_id: rel.object_id.clone(),
                    });
                }
            }
        }

        Ok(Ocel {
            event_types: self.event_types,
            object_types: self.object_types,
            events: self.events,
            objects: self.objects,
        })
    }
}
