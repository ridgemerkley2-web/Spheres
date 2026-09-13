//! Private retained-search storage. Authored graph indices fit in u32 today;
//! larger graphs keep the literal native vectors instead of truncating indices.

pub(super) trait PredecessorRead {
    fn predecessor(&self, at: usize) -> Option<(usize, usize)>;
}

impl PredecessorRead for [Option<(usize, usize)>] {
    fn predecessor(&self, at: usize) -> Option<(usize, usize)> { self[at] }
}
impl PredecessorRead for Vec<Option<(usize, usize)>> {
    fn predecessor(&self, at: usize) -> Option<(usize, usize)> { self[at] }
}

#[derive(Clone, Debug)]
pub(super) enum Predecessors {
    Compact(Vec<u64>),
    Native(Vec<Option<(usize, usize)>>),
}

impl Predecessors {
    // Reserve the all-ones node/edge indices, so all-ones u64 is unambiguously
    // None. A count equal to u32::MAX still has maximum index u32::MAX - 1.
    pub(super) fn fits(nodes: usize, edges: usize) -> bool {
        nodes <= u32::MAX as usize && edges <= u32::MAX as usize
    }

    pub(super) fn new(nodes: usize, edges: usize, compact: bool) -> Self {
        if compact && Self::fits(nodes, edges) { Self::Compact(vec![u64::MAX; nodes]) }
        else { Self::Native(vec![None; nodes]) }
    }

    pub(super) fn set(&mut self, at: usize, value: Option<(usize, usize)>) {
        match self {
            Self::Compact(rows) => {
                rows[at] = match value {
                    None => u64::MAX,
                    Some((node, edge)) => {
                        // Every caller passes indices from the same checked
                        // graph that selected this representation at creation.
                        debug_assert!(node < u32::MAX as usize && edge < u32::MAX as usize);
                        ((edge as u64) << 32) | node as u64
                    }
                };
            }
            Self::Native(rows) => rows[at] = value,
        }
    }

    pub(super) fn buffer_bytes(&self) -> usize {
        match self {
            Self::Compact(rows) => rows.capacity() * std::mem::size_of::<u64>(),
            Self::Native(rows) => rows.capacity() * std::mem::size_of::<Option<(usize, usize)>>(),
        }
    }

    #[cfg(test)]
    pub(super) fn to_native(&self) -> Vec<Option<(usize, usize)>> {
        let len = match self { Self::Compact(rows) => rows.len(), Self::Native(rows) => rows.len() };
        (0..len).map(|i| self.predecessor(i)).collect()
    }
}

impl PredecessorRead for Predecessors {
    fn predecessor(&self, at: usize) -> Option<(usize, usize)> {
        match self {
            Self::Compact(rows) => {
                let value = rows[at];
                if value == u64::MAX { None }
                else { Some(((value & u32::MAX as u64) as usize, (value >> 32) as usize)) }
            }
            Self::Native(rows) => rows[at],
        }
    }
}

#[derive(Clone, Debug)]
pub(super) enum SettlementRanks {
    Compact(Vec<u32>),
    Native(Vec<usize>),
}

impl SettlementRanks {
    pub(super) fn fits(nodes: usize) -> bool { nodes <= u32::MAX as usize }

    pub(super) fn new(nodes: usize, compact: bool) -> Self {
        if compact && Self::fits(nodes) { Self::Compact(vec![u32::MAX; nodes]) }
        else { Self::Native(vec![usize::MAX; nodes]) }
    }

    pub(super) fn get(&self, at: usize) -> usize {
        match self {
            Self::Compact(rows) => if rows[at] == u32::MAX { usize::MAX } else { rows[at] as usize },
            Self::Native(rows) => rows[at],
        }
    }

    pub(super) fn set(&mut self, at: usize, rank: usize) {
        match self {
            Self::Compact(rows) => {
                // Strict positive-edge Dijkstra settles a node at most once;
                // its rank is below the graph node count checked by new().
                debug_assert!(rank < u32::MAX as usize || rank == usize::MAX);
                rows[at] = if rank == usize::MAX { u32::MAX } else { rank as u32 };
            }
            Self::Native(rows) => rows[at] = rank,
        }
    }

    pub(super) fn buffer_bytes(&self) -> usize {
        match self {
            Self::Compact(rows) => rows.capacity() * std::mem::size_of::<u32>(),
            Self::Native(rows) => rows.capacity() * std::mem::size_of::<usize>(),
        }
    }

    #[cfg(test)]
    pub(super) fn to_native(&self) -> Vec<usize> {
        let len = match self { Self::Compact(rows) => rows.len(), Self::Native(rows) => rows.len() };
        (0..len).map(|i| self.get(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s08_compact_tree_storage_preserves_native_sentinels_indices_and_rank_bits() {
        let mut compact = Predecessors::new(7, 13, true);
        let mut native = vec![None; 7];
        let mut ranks = SettlementRanks::new(7, true);
        let mut native_ranks = vec![usize::MAX; 7];
        for (at, predecessor, rank) in [(0, None, 0), (3, Some((0, 12)), 1),
            (2, Some((3, 0)), 2), (6, Some((2, 11)), 3), (2, None, usize::MAX),
            (5, Some((6, 7)), 4)] {
            compact.set(at, predecessor); native[at] = predecessor;
            ranks.set(at, rank); native_ranks[at] = rank;
            assert_eq!(compact.to_native(), native);
            assert_eq!(ranks.to_native(), native_ranks);
            assert_eq!(compact.clone().to_native(), native);
            assert_eq!(ranks.clone().to_native(), native_ranks);
        }
        assert_eq!(compact.buffer_bytes(), 7 * std::mem::size_of::<u64>());
        assert_eq!(ranks.buffer_bytes(), 7 * std::mem::size_of::<u32>());
        // Exercise the encoding boundaries with a tiny backing vector rather
        // than allocating a fictitious four-billion-node graph.
        compact.set(0, Some((u32::MAX as usize - 1, u32::MAX as usize - 1)));
        assert_eq!(compact.predecessor(0), Some((u32::MAX as usize - 1, u32::MAX as usize - 1)));
        ranks.set(0, u32::MAX as usize - 1);
        assert_eq!(ranks.get(0), u32::MAX as usize - 1);
        ranks.set(0, usize::MAX);
        assert_eq!(ranks.get(0), usize::MAX);

        assert!(Predecessors::fits(u32::MAX as usize, u32::MAX as usize));
        assert!(SettlementRanks::fits(u32::MAX as usize));
        if let Some(oversized) = (u32::MAX as usize).checked_add(1) {
            assert!(!Predecessors::fits(oversized, 1));
            assert!(!Predecessors::fits(1, oversized));
            assert!(!SettlementRanks::fits(oversized));
            let mut wide = Predecessors::new(2, oversized, true);
            assert!(matches!(wide, Predecessors::Native(_)));
            wide.set(0, Some((usize::MAX - 1, usize::MAX)));
            assert_eq!(wide.predecessor(0), Some((usize::MAX - 1, usize::MAX)));
            assert_eq!(wide.predecessor(1), None);
        }
        let mut wide_ranks = SettlementRanks::new(2, false);
        assert!(matches!(wide_ranks, SettlementRanks::Native(_)));
        wide_ranks.set(0, usize::MAX - 1);
        assert_eq!(wide_ranks.get(0), usize::MAX - 1);
        assert_eq!(wide_ranks.get(1), usize::MAX);
        assert_eq!(wide_ranks.buffer_bytes(), 2 * std::mem::size_of::<usize>());
    }
}
