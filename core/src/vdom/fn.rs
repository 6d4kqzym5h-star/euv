use super::*;

/// Returns `true` if every node in the slice has a key.
///
/// Returns `false` if the slice is empty (an empty list
/// trivially has no keys; treating it as keyed would
/// cause a fallback when both sides are empty).
///
/// # Arguments
///
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
///
/// # Returns
///
/// - `bool` - A boolean.
pub fn all_have_keys(children: &[VirtualNode]) -> bool {
    !children.is_empty() && children.iter().all(VirtualNode::has_key)
}

/// Computes the diff between two virtual-DOM child lists.
///
/// Dispatches to `diff_keyed` when both lists have keys
/// on every node, otherwise falls back to `diff_positional`.
///
/// # Arguments
///
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
///
/// # Returns
///
/// - `Vec<DiffOp>` - A `Vec<DiffOp>` value.
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
///
/// # Arguments
///
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
///
/// # Returns
///
/// - `Vec<DiffOp>` - A `Vec<DiffOp>` value.
pub fn diff_keyed(old: &[VirtualNode], new: &[VirtualNode]) -> Vec<DiffOp> {
    let mut ops: Vec<DiffOp> = Vec::new();
    // Build a set of new keys for fast removal check.
    let mut new_key_set: HashSet<&str> = HashSet::with_capacity(new.len());
    for new_child in new.iter() {
        if let Some(key) = new_child.key() {
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
        let Some(key) = new_child.key() else {
            continue;
        };
        let existed_in_old: bool = old.iter().any(|old_child| old_child.key() == Some(key));
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
        let Some(key) = old_child.key() else {
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
///
/// # Arguments
///
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
/// - `&[VirtualNode]` - Shared reference to a `[VirtualNode]`.
///
/// # Returns
///
/// - `Vec<DiffOp>` - A `Vec<DiffOp>` value.
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
