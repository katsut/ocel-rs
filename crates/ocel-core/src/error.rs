use thiserror::Error;

/// Errors returned when building or validating an OCEL log.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OcelError {
    #[error("duplicate event id: {0}")]
    DuplicateEventId(String),

    #[error("duplicate object id: {0}")]
    DuplicateObjectId(String),

    #[error("event {event} has undeclared event type {event_type}")]
    UndeclaredEventType { event: String, event_type: String },

    #[error("object {object} has undeclared object type {object_type}")]
    UndeclaredObjectType { object: String, object_type: String },

    #[error("event {event} references unknown object {object}")]
    DanglingE2O { event: String, object: String },

    #[error("object {source_id} references unknown object {target_id}")]
    DanglingO2O {
        source_id: String,
        target_id: String,
    },
}
