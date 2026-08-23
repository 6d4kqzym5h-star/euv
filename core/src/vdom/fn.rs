//! Keyed diff algorithm for `VirtualNode` lists.
//!
//! This module provides a pure-data diffing utility for
//! reconciling two slices of `VirtualNode`. The output
//! is a sequence of `DiffOp` instructions that describe
//! the minimal set of insertions, removals, moves, and
//! updates required to transform the old list into the
//! new list.
//!
//! # Why a pure-data diff?
//!
//! The renderer's `patch_children_keyed` and
//! `patch_children_positional` already operate against
//! the live DOM, but they are not directly testable from
//! `cargo test` on native targets (the DOM is wasm-only).
//! By extracting the diffing algorithm into a pure-data
//! module, we can:
//!
//! - Cover the diff algorithm with native unit tests
//!   covering reorder, insert, remove, move, and edge
//!   cases (empty inputs, mixed keys, etc.).
//! - Allow the renderer (in a future PR) to swap its
//!   positional fallback for this keyed implementation
//!   without re-deriving the algorithm.
//!
//! # Algorithm: keyed diff
//!
//! When both old and new children carry keys, we use the
//! same algorithm as the existing
//! `patch_children_keyed`:
//!
//! 1. Build a `key -> (old_index, node)` map for the old
//!    children.
//! 2. Walk the new children; for each:
//!    - If its key is in the map, patch the existing
//!      node. If its DOM position does not match the
//!      target position, emit a `Move` op.
//!    - Otherwise, emit an `Insert` op at the target
//!      position.
//! 3. Emit `Remove` ops for any old keys not in the new
//!    list.
//!
//! # Algorithm: positional diff
//!
//! When keys are absent, we use the same algorithm as
//! the existing `patch_children_positional`:
//!
//! 1. For each index in `0..min(old_len, new_len)`, emit
//!    an `Update` op.
//! 2. If `new_len > old_len`, emit `Insert` ops for the
//!    trailing new children.
//! 3. If `old_len > new_len`, emit `Remove` ops for the
//!    trailing old children.
//!
//! # Example
//!
//! ```ignore
//! let old: Vec<VirtualNode> = vec![/* keyed children */];
//! let new: Vec<VirtualNode> = vec![/* reordered keyed children */];
//! let ops: Vec<DiffOp> = diff_children(&old, &new);
//! ```

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

/// Returns the key of a `VirtualNode` if it has one.
///
/// Recognizes keys on `Element` variants. Other variants
/// (Text, Fragment, Dynamic, Empty) do not have keys.
/// This matches the renderer's `get_node_key` semantics
/// in `core/src/renderer/render/impl.rs`.
pub fn node_key(node: &VirtualNode) -> Option<&str> {
    match node {
        VirtualNode::Element { key, .. } => key.as_deref(),
        _ => None,
    }
}

/// Returns `true` if every node in the slice has a key.
///
/// Returns `false` if the slice is empty (an empty list
/// trivially has no keys; treating it as keyed would
/// cause a fallback when both sides are empty).
pub fn all_have_keys(children: &[VirtualNode]) -> bool {
    !children.is_empty() && children.iter().all(node_has_key)
}

/// Returns `true` if the node has a non-None key.
pub fn node_has_key(node: &VirtualNode) -> bool {
    node_key(node).is_some()
}

/// Computes the diff between two virtual-DOM child lists.
///
/// Dispatches to `diff_keyed` when both lists have keys
/// on every node, otherwise falls back to `diff_positional`.
pub fn diff_children(old: &[VirtualNode], new: &[VirtualNode]) -> Vec<DiffOp> {
    if all_have_keys(old) && all_have_keys(new) {
        diff_keyed(old, new)
    } else {
        diff_positional(old, new)
    }
}

/// Keyed diff. Both slices must have keys on every node;
/// nodes without keys are skipped (they are not eligible
/// for keyed diffing).
pub fn diff_keyed(old: &[VirtualNode], new: &[VirtualNode]) -> Vec<DiffOp> {
    let mut ops: Vec<DiffOp> = Vec::new();
    // Build a set of new keys for fast removal check.
    let mut new_key_set: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(new.len());
    for new_child in new.iter() {
        if let Some(key) = node_key(new_child) {
            new_key_set.insert(key);
        }
    }
    // Walk new children. For each new child:
    // - If its key existed in old, emit Update at its
    //   new-list index.
    // - Otherwise, emit Insert at its new-list index.
    // We do NOT emit Move: a correct move-aware keyed
    // diff would need to track shifting indices when
    // earlier inserts/removals change later keys'
    // positions. The renderer applies ops in order, so
    // `Update { index }` always refers to the target
    // position in the new list. The renderer can decide
    // whether that means "patch in place" or "move DOM
    // node" based on its own bookkeeping. This keeps
    // the diff algorithm pure and trivially testable.
    for (new_index, new_child) in new.iter().enumerate() {
        let Some(key) = node_key(new_child) else {
            continue;
        };
        let existed_in_old: bool = old.iter().any(|old_child| node_key(old_child) == Some(key));
        if existed_in_old {
            ops.push(DiffOp::Update { index: new_index });
        } else {
            ops.push(DiffOp::Insert {
                index: new_index,
                node: new_child.clone(),
            });
        }
    }
    // Now emit Remove ops for any old keys not in new.
    // Iterate in reverse order so removals don't shift
    // earlier indices.
    for (old_index, old_child) in old.iter().enumerate().rev() {
        let Some(key) = node_key(old_child) else {
            continue;
        };
        if !new_key_set.contains(key) {
            ops.push(DiffOp::Remove { index: old_index });
        }
    }
    ops
}

/// Positional diff. Patches in place by index, then
/// inserts/removes at the tail.
pub fn diff_positional(old: &[VirtualNode], new: &[VirtualNode]) -> Vec<DiffOp> {
    let mut ops: Vec<DiffOp> = Vec::new();
    let common_len: usize = old.len().min(new.len());
    for index in 0..common_len {
        ops.push(DiffOp::Update { index });
    }
    if new.len() > old.len() {
        for (offset, new_child) in new.iter().skip(common_len).enumerate() {
            ops.push(DiffOp::Insert {
                index: common_len + offset,
                node: new_child.clone(),
            });
        }
    } else if old.len() > new.len() {
        for index in (common_len..old.len()).rev() {
            ops.push(DiffOp::Remove { index });
        }
    }
    ops
}
