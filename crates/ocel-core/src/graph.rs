//! Object interaction graph and connected components.
//!
//! Two objects are connected when they co-occur in the `E2O` relationships of
//! the same event, or when an `O2O` relationship links them (either direction).
//! This is the standard structure for OCEL-aware filtering and sampling, where
//! events, objects, and relations are interdependent and cannot be dropped
//! independently (Berti 2022, arXiv:2205.01428).
//!
//! Built with `std` only (adjacency lists + BFS); deterministic: the same log
//! always yields the same component decomposition and ordering.

use std::collections::BTreeMap;

use crate::model::Ocel;

/// An undirected interaction graph over the objects of an [`Ocel`] log.
#[derive(Debug)]
pub struct ObjectGraph<'a> {
    /// Sorted, deduplicated object ids; index positions are node handles.
    ids: Vec<&'a str>,
    index: BTreeMap<&'a str, usize>,
    /// Sorted, deduplicated adjacency lists.
    adjacency: Vec<Vec<usize>>,
}

impl Ocel {
    /// Build the object interaction graph of this log.
    ///
    /// Unknown object ids in relationships (possible in unvalidated logs) are
    /// skipped rather than treated as nodes.
    #[must_use]
    pub fn object_graph(&self) -> ObjectGraph<'_> {
        let mut ids: Vec<&str> = self.objects.iter().map(|o| o.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        let index: BTreeMap<&str, usize> = ids.iter().enumerate().map(|(i, id)| (*id, i)).collect();
        let mut adjacency = vec![Vec::new(); ids.len()];

        // (a) co-occurrence in the same event's E2O relationships
        for event in &self.events {
            let mut members: Vec<usize> = event
                .relationships
                .iter()
                .filter_map(|r| index.get(r.object_id.as_str()).copied())
                .collect();
            members.sort_unstable();
            members.dedup();
            for (i, &a) in members.iter().enumerate() {
                for &b in &members[i + 1..] {
                    adjacency[a].push(b);
                    adjacency[b].push(a);
                }
            }
        }

        // (b) O2O relationships (undirected)
        for object in &self.objects {
            let Some(&a) = index.get(object.id.as_str()) else {
                continue;
            };
            for rel in &object.relationships {
                if let Some(&b) = index.get(rel.object_id.as_str()) {
                    if a != b {
                        adjacency[a].push(b);
                        adjacency[b].push(a);
                    }
                }
            }
        }

        for list in &mut adjacency {
            list.sort_unstable();
            list.dedup();
        }
        ObjectGraph {
            ids,
            index,
            adjacency,
        }
    }
}

impl<'a> ObjectGraph<'a> {
    /// Number of objects (nodes) in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Whether the graph has no objects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Whether `object_id` is a node of the graph.
    #[must_use]
    pub fn contains(&self, object_id: &str) -> bool {
        self.index.contains_key(object_id)
    }

    /// The objects directly interacting with `object_id`, in sorted order.
    /// Empty when the object is unknown or isolated.
    pub fn neighbors(&self, object_id: &str) -> impl Iterator<Item = &'a str> + '_ {
        self.index
            .get(object_id)
            .into_iter()
            .flat_map(move |&i| self.adjacency[i].iter().map(move |&j| self.ids[j]))
    }

    /// Connected components as sorted object-id lists.
    ///
    /// Deterministic: components are ordered by their smallest object id, and
    /// ids are sorted within each component. Every object appears exactly once.
    #[must_use]
    pub fn connected_components(&self) -> Vec<Vec<&'a str>> {
        let mut seen = vec![false; self.ids.len()];
        let mut components = Vec::new();
        for start in 0..self.ids.len() {
            if seen[start] {
                continue;
            }
            seen[start] = true;
            let mut queue = vec![start];
            let mut component = Vec::new();
            while let Some(node) = queue.pop() {
                component.push(node);
                for &next in &self.adjacency[node] {
                    if !seen[next] {
                        seen[next] = true;
                        queue.push(next);
                    }
                }
            }
            component.sort_unstable();
            components.push(component.into_iter().map(|i| self.ids[i]).collect());
        }
        components
    }
}
