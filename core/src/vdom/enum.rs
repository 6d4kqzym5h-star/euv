use super::*;

/// A single diff operation that transforms one virtual-
/// DOM list into another.
///
/// Operations are emitted in the order they should be
/// applied. Indices refer to positions in the *current*
/// list (after prior ops have been applied).
#[derive(Clone, Debug, PartialEq)]
pub enum DiffOp {
    /// Insert a new node at the given index. The index is
    /// the position in the new list.
    Insert {
        /// The target index in the new list.
        index: usize,
        /// The new node being inserted.
        node: VirtualNode,
    },
    /// Remove the node at the given index from the old
    /// list.
    Remove {
        /// The index of the node to remove.
        index: usize,
    },
    /// Move the node currently at `from` to `to`.
    Move {
        /// The current index of the node.
        from: usize,
        /// The target index.
        to: usize,
    },
    /// Patch the node at the given index in place (both
    /// old and new lists reference the same node, so the
    /// index is the same in both).
    Update {
        /// The index of the node to patch.
        index: usize,
    },
}
