use super::*;

/// Inherent implementation of [`HmrState`].
impl HmrState {
    /// Creates a new empty `HmrState`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an `HmrState` from an iterator of
    /// `(key, value)` pairs. Later pairs overwrite
    /// earlier ones for the same key.
    ///
    /// # Arguments
    ///
    /// - `I` - A generic type parameter.
    pub fn from_entries<I>(entries: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        let mut state: Self = Self::new();
        for (key, value) in entries {
            state.entries.insert(key, value);
        }
        state
    }

    /// Sets a key-value pair. Overwrites any existing
    /// value for the key.
    ///
    /// # Arguments
    ///
    /// - `K` - A `K` parameter.
    /// - `V` - A `V` parameter.
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.entries.insert(key.into(), value.into());
    }

    /// Returns the value for the given key, or `None`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<str>` - The current value (or a snapshot thereof).
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s: &String| s.as_str())
    }

    /// Removes the entry for the given key, returning
    /// the previous value if any.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - `Some(...)` on success, `None` otherwise.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key)
    }

    /// Removes every entry.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the number of entries.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of items in the collection.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the state is empty.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns `true` if the state contains the given
    /// key.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Returns an iterator over the entries.
    ///
    /// # Returns
    ///
    /// - `impl Iterator<Item` - A `impl Iterator<Item` value.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(k, v): (&String, &String)| (k.as_str(), v.as_str()))
    }
}
