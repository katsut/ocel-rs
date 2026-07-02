//! OCEL-aware filtering.
//!
//! Events, objects, and relations are interdependent in OCEL: dropping an
//! object silently changes the meaning of every event that references it
//! (Berti 2022, arXiv:2205.01428). Every operation here therefore returns a
//! consistent sub-log — **the result always passes [`Ocel::validate`]** — by
//! stripping relationships that would dangle.
//!
//! Semantics follow `PM4Py` where applicable:
//! - [`Ocel::filter_events`] keeps matching events and only the objects they
//!   reference (isolated objects are dropped).
//! - [`Ocel::filter_object_types`] keeps objects of the given types and only
//!   the events still related to at least one kept object.
//!
//! All type declarations are preserved as-is (unused declarations are valid).

use std::collections::BTreeSet;

use chrono::{DateTime, Utc};

use crate::model::{Event, Ocel};

impl Ocel {
    /// Keep the events matching `pred`, and only the objects they reference.
    ///
    /// `O2O` relationships are restricted to the kept objects. Objects that no
    /// kept event references (including isolated objects) are dropped.
    #[must_use]
    pub fn filter_events<F>(&self, pred: F) -> Ocel
    where
        F: Fn(&Event) -> bool,
    {
        let kept_events: Vec<&Event> = self.events.iter().filter(|e| pred(e)).collect();
        let kept_objects: BTreeSet<&str> = kept_events
            .iter()
            .flat_map(|e| e.relationships.iter().map(|r| r.object_id.as_str()))
            .collect();
        self.subset(|e| pred(e), |id| kept_objects.contains(id), KeepEvents::All)
    }

    /// Keep the events whose type is in `names` (and the objects they reference).
    #[must_use]
    pub fn filter_event_types(&self, names: &[&str]) -> Ocel {
        self.filter_events(|e| names.contains(&e.event_type.as_str()))
    }

    /// Keep the events with `from <= time <= to` (and the objects they reference).
    #[must_use]
    pub fn filter_time_range(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Ocel {
        self.filter_events(|e| e.time >= from && e.time <= to)
    }

    /// Keep the objects whose type is in `names`, and only the events still
    /// related to at least one kept object.
    ///
    /// `E2O` relationships to dropped objects are stripped from the kept
    /// events; events with no remaining relationship are dropped.
    #[must_use]
    pub fn filter_object_types(&self, names: &[&str]) -> Ocel {
        let kept_objects: BTreeSet<&str> = self
            .objects
            .iter()
            .filter(|o| names.contains(&o.object_type.as_str()))
            .map(|o| o.id.as_str())
            .collect();
        self.subset(
            |_| true,
            |id| kept_objects.contains(id),
            KeepEvents::RelatedToKeptObjects,
        )
    }

    /// Build a consistent sub-log from event/object predicates.
    ///
    /// Relationships pointing outside the kept object set are stripped, so the
    /// result validates whenever `self` does.
    pub(crate) fn subset<E, O>(&self, keep_event: E, keep_object: O, mode: KeepEvents) -> Ocel
    where
        E: Fn(&Event) -> bool,
        O: Fn(&str) -> bool,
    {
        let events = self
            .events
            .iter()
            .filter(|e| keep_event(e))
            .map(|e| {
                let mut event = e.clone();
                event
                    .relationships
                    .retain(|r| keep_object(r.object_id.as_str()));
                event
            })
            .filter(|e| match mode {
                KeepEvents::All => true,
                KeepEvents::RelatedToKeptObjects => !e.relationships.is_empty(),
            })
            .collect();
        let objects = self
            .objects
            .iter()
            .filter(|o| keep_object(o.id.as_str()))
            .map(|o| {
                let mut object = o.clone();
                object
                    .relationships
                    .retain(|r| keep_object(r.object_id.as_str()));
                object
            })
            .collect();
        Ocel {
            event_types: self.event_types.clone(),
            object_types: self.object_types.clone(),
            events,
            objects,
        }
    }
}

/// Whether `subset` keeps every event passing the predicate, or only those
/// still related to at least one kept object.
#[derive(Debug, Clone, Copy)]
pub(crate) enum KeepEvents {
    All,
    RelatedToKeptObjects,
}
