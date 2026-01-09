use std::time::Instant;

pub struct Entry<V> {
    pub value: V,
    pub expires_at: Instant,
}