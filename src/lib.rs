mod entry;
mod cache;
mod time;

pub use cache::Cache;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn basic_insert_and_get() {
        let mut cache = Cache::new();

        cache.insert("a", 42, Duration::from_secs(10));

        assert_eq!(cache.get(&"a"), Some(&42));
    }

    #[test]
    fn expires_after_ttl() {
        let mut cache = Cache::new();

        cache.insert("a", 42, Duration::from_secs(0));

        assert_eq!(cache.get(&"a"), None);
    }

    #[test]
    fn cleanup_removes_expired() {
        let mut cache = Cache::new();

        cache.insert("a", 1, Duration::from_secs(0));
        cache.insert("b", 2, Duration::from_secs(10));

        cache.cleanup();

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
    }
}
