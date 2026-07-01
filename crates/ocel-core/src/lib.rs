//! OCEL 2.0 core data model and I/O.
//!
//! The in-memory model is OCEL 2.0-native (see ADR 0001). ETL intermediate
//! representations live in downstream crates and convert into [`Ocel`] via the
//! [`OcelBuilder`] gate.

pub mod error;
pub mod io;
pub mod model;

pub use error::OcelError;
pub use model::{
    AttrType, AttrValue, AttributeDefinition, E2ORelation, Event, EventAttribute, EventColumns,
    EventType, O2ORelation, Object, ObjectAttribute, ObjectType, Ocel, OcelBuilder, Relationship,
};
