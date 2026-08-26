use super::*;

#[test]
fn new_cache_is_empty() {
    let cache: LruCache<String, i32> = LruCache::new(10);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert!(!cache.is_full());
    // `capacity` is `pub(crate)` on the struct; only the
    // framework's own code (which has access to the
    // private `get_capacity`) can inspect it. External
    // tests verify behaviour (the cache holds 0 entries
    // at construction) instead of the capacity value.
}

#[test]
fn put_inserts_entry() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    assert_eq!(cache.len(), 1);
    assert!(!cache.is_empty());
    assert_eq!(cache.peek(&"a".to_string()), Some(&1));
}

#[test]
fn get_returns_value_and_updates_recency() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    assert_eq!(cache.get(&"a".to_string()), Some(&1));
    let keys: Vec<&String> = cache.keys().collect();
    assert_eq!(keys, vec![&"a".to_string(), &"b".to_string()]);
}

#[test]
fn put_existing_key_replaces_value_and_promotes() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("a".to_string(), 100);
    assert_eq!(cache.peek(&"a".to_string()), Some(&100));
    assert_eq!(cache.len(), 2);
    let keys: Vec<&String> = cache.keys().collect();
    assert_eq!(keys, vec![&"a".to_string(), &"b".to_string()]);
}

#[test]
fn put_evicts_least_recently_used_when_full() {
    let mut cache: LruCache<String, i32> = LruCache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    let evicted: Option<(String, i32)> = cache.put("c".to_string(), 3);
    assert_eq!(evicted, Some(("a".to_string(), 1)));
    assert_eq!(cache.len(), 2);
    assert!(!cache.contains(&"a".to_string()));
    assert!(cache.contains(&"b".to_string()));
    assert!(cache.contains(&"c".to_string()));
}

#[test]
fn get_promotes_key_so_it_is_not_evicted_next() {
    let mut cache: LruCache<String, i32> = LruCache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    let _ = cache.get(&"a".to_string());
    let evicted: Option<(String, i32)> = cache.put("c".to_string(), 3);
    assert_eq!(evicted, Some(("b".to_string(), 2)));
    assert!(cache.contains(&"a".to_string()));
    assert!(cache.contains(&"c".to_string()));
}

#[test]
fn remove_drops_entry() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    let removed: Option<i32> = cache.remove(&"a".to_string());
    assert_eq!(removed, Some(1));
    assert_eq!(cache.len(), 0);
    assert!(!cache.contains(&"a".to_string()));
}

#[test]
fn remove_absent_key_returns_none() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    assert_eq!(cache.remove(&"a".to_string()), None);
}

#[test]
fn remove_drops_order_entry_too() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.remove(&"a".to_string());
    cache.put("a".to_string(), 3);
    let keys: Vec<&String> = cache.keys().collect();
    assert_eq!(keys, vec![&"a".to_string(), &"b".to_string()]);
}

#[test]
fn clear_drops_everything() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn contains_works() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    assert!(cache.contains(&"a".to_string()));
    assert!(!cache.contains(&"b".to_string()));
}

#[test]
fn contains_does_not_update_recency() {
    let mut cache: LruCache<String, i32> = LruCache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    assert!(cache.contains(&"a".to_string()));
    let evicted: Option<(String, i32)> = cache.put("c".to_string(), 3);
    assert_eq!(evicted, Some(("a".to_string(), 1)));
}

#[test]
fn capacity_zero_silently_drops_puts() {
    let mut cache: LruCache<String, i32> = LruCache::new(0);
    let evicted: Option<(String, i32)> = cache.put("a".to_string(), 1);
    assert_eq!(evicted, None);
    assert_eq!(cache.len(), 0);
}

#[test]
fn capacity_one_holds_exactly_one_entry() {
    let mut cache: LruCache<String, i32> = LruCache::new(1);
    cache.put("a".to_string(), 1);
    assert!(cache.is_full());
    let evicted: Option<(String, i32)> = cache.put("b".to_string(), 2);
    assert_eq!(evicted, Some(("a".to_string(), 1)));
    assert_eq!(cache.len(), 1);
    assert!(!cache.contains(&"a".to_string()));
    assert!(cache.contains(&"b".to_string()));
}

#[test]
fn iter_returns_mru_first() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    let entries: Vec<(&String, &i32)> = cache.iter().collect();
    assert_eq!(
        entries,
        vec![
            (&"c".to_string(), &3),
            (&"b".to_string(), &2),
            (&"a".to_string(), &1),
        ]
    );
}

#[test]
fn keys_returns_mru_first() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    let keys: Vec<&String> = cache.keys().collect();
    assert_eq!(
        keys,
        vec![&"c".to_string(), &"b".to_string(), &"a".to_string(),]
    );
}

#[test]
fn values_returns_mru_first() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    let values: Vec<&i32> = cache.values().collect();
    assert_eq!(values, vec![&2, &1]);
}

#[test]
fn resize_smaller_evicts_lru() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    cache.resize(2);
    assert_eq!(cache.len(), 2);
    assert!(cache.contains(&"c".to_string()));
    assert!(cache.contains(&"b".to_string()));
    assert!(!cache.contains(&"a".to_string()));
}

#[test]
fn resize_larger_is_no_eviction() {
    let mut cache: LruCache<String, i32> = LruCache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.resize(10);
    // capacity is `pub(crate)`; verify the resize side-effect
    // by inserting a third entry that would have evicted at
    // capacity=2 but must survive at the new capacity.
    cache.put("c".to_string(), 3);
    assert_eq!(cache.len(), 3);
    assert!(cache.contains(&"a".to_string()));
    assert!(cache.contains(&"b".to_string()));
    assert!(cache.contains(&"c".to_string()));
}

#[test]
fn resize_zero_evicts_everything() {
    let mut cache: LruCache<String, i32> = LruCache::new(10);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.resize(0);
    assert_eq!(cache.len(), 0);
}

#[test]
fn peek_does_not_update_recency() {
    let mut cache: LruCache<String, i32> = LruCache::new(2);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    let _ = cache.peek(&"a".to_string());
    let evicted: Option<(String, i32)> = cache.put("c".to_string(), 3);
    assert_eq!(evicted, Some(("a".to_string(), 1)));
}

#[test]
fn many_evictions_follow_lru_order() {
    let mut cache: LruCache<String, i32> = LruCache::new(3);
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3);
    cache.put("d".to_string(), 4); // evicts a
    cache.put("e".to_string(), 5); // evicts b
    assert!(!cache.contains(&"a".to_string()));
    assert!(!cache.contains(&"b".to_string()));
    assert!(cache.contains(&"c".to_string()));
    assert!(cache.contains(&"d".to_string()));
    assert!(cache.contains(&"e".to_string()));
}

#[test]
fn integer_key_works() {
    let mut cache: LruCache<i32, String> = LruCache::new(2);
    cache.put(1, String::from("one"));
    cache.put(2, String::from("two"));
    assert_eq!(cache.get(&1), Some(&String::from("one")));
    let evicted: Option<(i32, String)> = cache.put(3, String::from("three"));
    assert_eq!(evicted, Some((2, String::from("two"))));
}
