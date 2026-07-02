//! Connected-components sampling.
//!
//! OCEL logs cannot be sampled event-by-event or object-by-object without
//! changing the meaning of the remaining data (Berti 2022, arXiv:2205.01428).
//! The standard approach is to sample whole connected components of the
//! [object interaction graph](crate::graph::ObjectGraph): a component is closed
//! under event co-occurrence, so every kept event keeps all of its objects and
//! the result always passes [`Ocel::validate`].
//!
//! Deterministic by construction (components are ordered by their smallest
//! object id); callers wanting randomness can shuffle indices themselves and
//! use [`Ocel::filter_components`].

use std::collections::BTreeSet;

use crate::filter::KeepEvents;
use crate::model::Ocel;

impl Ocel {
    /// Keep the first `n` connected components (ordered by smallest object id).
    ///
    /// Events not related to any kept object — including events with no
    /// relationships at all — are dropped.
    #[must_use]
    pub fn sample_components(&self, n: usize) -> Ocel {
        let components = self.object_graph().connected_components();
        self.keep_components(components.iter().take(n))
    }

    /// Keep the components selected by `pred`, which receives each component's
    /// sorted object ids.
    #[must_use]
    pub fn filter_components<F>(&self, pred: F) -> Ocel
    where
        F: Fn(&[&str]) -> bool,
    {
        let components = self.object_graph().connected_components();
        self.keep_components(components.iter().filter(|c| pred(c)))
    }

    fn keep_components<'a, I>(&self, chosen: I) -> Ocel
    where
        I: Iterator<Item = &'a Vec<&'a str>>,
    {
        let kept: BTreeSet<&str> = chosen.flatten().copied().collect();
        self.subset(
            |_| true,
            |id| kept.contains(id),
            KeepEvents::RelatedToKeptObjects,
        )
    }
}
