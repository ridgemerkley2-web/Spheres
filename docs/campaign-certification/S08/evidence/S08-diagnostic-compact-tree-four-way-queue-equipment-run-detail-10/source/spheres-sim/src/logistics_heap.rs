//! A four-way heap for the retained clearing frontier. The original Visit
//! ordering still decides every pop, including equal-cost node ties. Duplicate
//! visits remain in the heap so strict relaxation and stale-pop handling stay
//! in the caller. The native pair search continues to use BinaryHeap.

use super::Visit;

#[derive(Clone, Default)]
pub(super) struct SearchQueue {
    entries: Vec<Visit>,
}

impl SearchQueue {
    pub(super) fn new() -> Self { Self::default() }
    pub(super) fn len(&self) -> usize { self.entries.len() }
    pub(super) fn capacity(&self) -> usize { self.entries.capacity() }

    pub(super) fn push(&mut self, visit: Visit) {
        let mut at = self.entries.len();
        self.entries.push(visit);
        while at > 0 {
            let parent = (at - 1) / 4;
            if visit <= self.entries[parent] { break; }
            self.entries[at] = self.entries[parent];
            at = parent;
        }
        self.entries[at] = visit;
    }

    pub(super) fn pop(&mut self) -> Option<Visit> {
        let last = self.entries.pop()?;
        if self.entries.is_empty() { return Some(last); }
        let first = self.entries[0];
        let mut at = 0;
        // A Vec<Visit> cannot allocate enough elements for this multiplication
        // to overflow: Visit is at least four times larger than one byte.
        while at * 4 + 1 < self.entries.len() {
            let child = at * 4 + 1;
            let end = (child + 4).min(self.entries.len());
            let mut best = child;
            for candidate in child + 1..end {
                if self.entries[candidate] > self.entries[best] { best = candidate; }
            }
            self.entries[at] = self.entries[best];
            at = best;
        }
        // As with the standard binary heap, move the hole to the bottom first
        // and then insert the old tail. Visits are Copy; no unsafe hole state,
        // new per-node index array or additional allocation is needed.
        while at > 0 {
            let parent = (at - 1) / 4;
            if last <= self.entries[parent] { break; }
            self.entries[at] = self.entries[parent];
            at = parent;
        }
        self.entries[at] = last;
        Some(first)
    }

    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool { self.entries.is_empty() }

    #[cfg(test)]
    pub(super) fn into_sorted_vec(mut self) -> Vec<Visit> {
        let mut sorted = Vec::with_capacity(self.len());
        while let Some(visit) = self.pop() { sorted.push(visit); }
        sorted.reverse();
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    fn values(visits: Vec<Visit>) -> Vec<(u64, usize)> {
        visits.into_iter().map(|visit| (visit.cost, visit.node)).collect()
    }

    #[test]
    fn s08_four_way_queue_matches_native_priority_through_interleaved_work() {
        let mut actual = SearchQueue::new();
        let mut native = BinaryHeap::new();
        let mut seed = 0x85e3_a701_1337_cafe_u64;
        for step in 0..40_000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            if seed % 5 < 3 {
                // Many ties, repeated nodes and obsolete higher-cost entries.
                let visit = Visit { cost: (seed >> 17) % 79, node: (seed >> 32) as usize % 367 };
                actual.push(visit);
                native.push(visit);
            } else {
                assert_eq!(actual.pop().map(|v| (v.cost, v.node)), native.pop().map(|v| (v.cost, v.node)));
            }
            assert_eq!(actual.len(), native.len());
            if step % 997 == 0 {
                assert_eq!(values(actual.clone().into_sorted_vec()), values(native.clone().into_sorted_vec()));
            }
        }
        assert_eq!(values(actual.into_sorted_vec()), values(native.into_sorted_vec()));
    }

    #[test]
    fn s08_four_way_queue_preserves_ties_extremes_and_partial_child_groups() {
        for size in 0..70 {
            let mut actual = SearchQueue::new();
            let mut native = BinaryHeap::new();
            for index in (0..size).rev() {
                for cost in [u64::MAX, 0, 1, 1, u64::MAX - 1] {
                    let visit = Visit { cost, node: if index % 3 == 0 { usize::MAX } else { index } };
                    actual.push(visit);
                    native.push(visit);
                }
            }
            while !native.is_empty() {
                assert_eq!(actual.pop().map(|v| (v.cost, v.node)), native.pop().map(|v| (v.cost, v.node)));
            }
            assert!(actual.is_empty());
            assert!(actual.pop().is_none());
            actual.push(Visit { cost: 0, node: 0 });
            assert_eq!(actual.pop().map(|v| (v.cost, v.node)), Some((0, 0)));
            assert!(actual.pop().is_none());
        }
    }
}
