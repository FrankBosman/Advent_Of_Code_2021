use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Priority {
    pub(crate) cost: usize,
    pub(crate) value: usize,
}

impl Priority {
    pub(crate) fn new(cost: usize, value: usize) -> Priority {
        Self { cost, value }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.value.cmp(&other.value))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Priority{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}