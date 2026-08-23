//! LRU cache implementation.
//!
//! Backed by a `HashMap<K, V>` for O(1) lookup and a
//! `VecDeque<K>` for LRU order. The `VecDeque` stores
//! keys in MRU-first order — the front is the most
//! recently used, the back is the least recently used.
//!
//! # Why `VecDeque` and not a linked list?
//!
//! For typical cache sizes (≤ 100 entries), `VecDeque`
//! has better cache locality than a heap-allocated
//! linked list. The `remove` operation is O(n) per entry,
//! which is fine at this scale — the bottleneck is the
//! `HashMap` lookup, not the order-bookkeeping.
use std::iter::Iterator;
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
    capacity: usize,
    /// The current entries, keyed by K.
    map: std::collections::HashMap<K, V>,
    /// The MRU-first order. Front = most recently used,
    /// back = least recently used.
    order: std::collections::VecDeque<K>,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
{
    /// Creates a new LRU cache with the given capacity.
    /// Capacity of 0 is allowed; the cache will accept
    /// no entries.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            order: std::collections::VecDeque::new(),
        }
    }

    /// Returns the maximum number of entries the cache
    /// can hold before eviction kicks in.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the current number of entries in the
    /// cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns `true` if the cache is at capacity.
    pub fn is_full(&self) -> bool {
        self.map.len() >= self.capacity
    }

    /// Returns `true` if the cache contains a value for
    /// the given key. Does NOT update the recency (use
    /// `get` for that).
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Returns the value for the given key, updating the
    /// recency so the entry becomes the most-recently-
    /// used.
    ///
    /// Returns `None` if the key is not in the cache.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Promote the key to the front of the
            // order deque. Remove its existing position
            // first (if any) to avoid duplicates.
            self.order.retain(|k: &K| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// Returns the value for the given key without
    /// updating the recency. Useful for "is this cached?"
    /// checks that should not affect eviction order.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    /// Inserts a key-value pair into the cache. If the
    /// key is already present, the existing value is
    /// replaced (and the entry becomes the most-
    /// recently-used). If the cache is at capacity and
    /// the key is new, the least-recently-used entry is
    /// evicted first.
    ///
    /// Returns the evicted entry, if any.
    pub fn put(&mut self, key: K, value: V) -> Option<(K, V)> {
        // Capacity of 0 — silently drop.
        if self.capacity == 0 {
            return None;
        }
        // Updating an existing key.
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            // Promote the key to the front of the
            // order deque. Remove its existing position
            // first (if any) to avoid duplicates.
            self.order.retain(|k: &K| k != &key);
            self.order.push_front(key);
            return None;
        }
        // Inserting a new key. Evict if at capacity.
        let evicted: Option<(K, V)> = if self.map.len() >= self.capacity {
            let victim_key: K = self.order.pop_back()?;
            let victim_value: V = self.map.remove(&victim_key)?;
            Some((victim_key, victim_value))
        } else {
            None
        };
        self.map.insert(key.clone(), value);
        self.order.push_front(key);
        evicted
    }

    /// Removes the entry for the given key, returning
    /// the removed value if any.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k: &K| k != key);
        self.map.remove(key)
    }

    /// Removes every entry from the cache.
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Returns an iterator over the entries in
    /// most-recently-used-first order.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        // We can't return the VecDeque order directly
        // because the entries would be in order-deque
        // order, not MRU-first order. Actually they
        // ARE in MRU-first order — the VecDeque's
        // front is MRU. So iterating and mapping through
        // the map gives us MRU-first order.
        self.order
            .iter()
            .filter_map(|k: &K| self.map.get_key_value(k))
    }

    /// Returns an iterator over the keys in
    /// most-recently-used-first order.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.order.iter()
    }

    /// Returns an iterator over the values in
    /// most-recently-used-first order.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.order.iter().filter_map(|k: &K| self.map.get(k))
    }

    /// Resizes the cache to a new capacity.
    ///
    /// If the new capacity is smaller than the current
    /// size, the least-recently-used entries are
    /// evicted until the cache fits.
    pub fn resize(&mut self, new_capacity: usize) {
        self.capacity = new_capacity;
        while self.map.len() > self.capacity {
            if let Some(victim_key) = self.order.pop_back() {
                self.map.remove(&victim_key);
            } else {
                break;
            }
        }
    }
}
