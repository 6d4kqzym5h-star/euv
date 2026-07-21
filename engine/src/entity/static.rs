use super::*;

/// A monotonically increasing counter used to assign unique IDs to entities.
pub(crate) static NEXT_ENTITY_ID: AtomicU64 = AtomicU64::new(1);
