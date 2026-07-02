use crate::model::{Event, EventType, Object, ObjectType, Ocel};
use crate::validate::Violation;

/// A fallible builder that validates structural integrity on [`build`](OcelBuilder::build).
///
/// This is the core-level gate for ETL-style construction (ADR 0001): the raw
/// parts are accumulated freely and checked once, up front, by the same
/// validation used for [`Ocel::validate`].
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
    /// Returns every [`Violation`] found (duplicate ids, undeclared types or
    /// attributes, dangling `E2O` / `O2O` references).
    pub fn build(self) -> Result<Ocel, Vec<Violation>> {
        let ocel = Ocel {
            event_types: self.event_types,
            object_types: self.object_types,
            events: self.events,
            objects: self.objects,
        };
        ocel.validate()?;
        Ok(ocel)
    }
}
