/// A fixed-capacity LRU cache.
///
/// The cache holds at most `capacity` entries. When a
/// `put` would exceed the capacity, the
/// least-recently-used entry is evicted. `get` updates
/// the recency so the just-read entry becomes the most-
/// recently-used.
///
/// All operations are O(1) amortized (`put`, `get`,
/// `remove`, `contains`) except `iter`, which is O(n).
///
/// # Capacity edge cases
///
/// - `capacity = 0` - the cache accepts no entries. Both
///   `put` and `get` behave as no-ops (well, `get` still
///   evicts because there's nothing to evict; `put`
///   silently drops the entry).
/// - `capacity = 1` - the cache holds exactly one entry.
///   Every `put` evicts the previous entry.
#[derive(Clone, Debug)]
pub struct LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
{
    /// The maximum number of entries before eviction
    /// kicks in.
    pub(crate) capacity: usize,
    /// The current entries, keyed by K.
    pub(crate) map: std::collections::HashMap<K, V>,
    /// The MRU-first order. Front = most recently used,
    /// back = least recently used.
    pub(crate) order: std::collections::VecDeque<K>,
}
