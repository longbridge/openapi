use std::{
    collections::HashMap,
    hash::Hash,
    time::{Duration, Instant},
};

use futures_util::Future;
use tokio::sync::Mutex;

struct Item<V> {
    deadline: Instant,
    value: V,
}

struct Inner<K, V> {
    timeout: Duration,
    values: HashMap<K, Item<V>>,
}

pub(crate) struct CacheWithKey<K, V> {
    inner: Mutex<Inner<K, V>>,
}

impl<K, V> CacheWithKey<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub(crate) fn new(timeout: Duration) -> Self {
        CacheWithKey {
            inner: Mutex::new(Inner {
                timeout,
                values: HashMap::new(),
            }),
        }
    }

    pub(crate) async fn get_or_update<F, Fut, Err>(&self, key: K, f: F) -> Result<V, Err>
    where
        F: FnOnce(K) -> Fut,
        Fut: Future<Output = Result<V, Err>>,
    {
        let mut inner = self.inner.lock().await;
        match inner.values.get(&key) {
            Some(Item { deadline, value }) if deadline > &Instant::now() => Ok(value.clone()),
            _ => {
                let value = f(key.clone()).await?;
                let deadline = Instant::now() + inner.timeout;
                inner.values.insert(
                    key,
                    Item {
                        deadline,
                        value: value.clone(),
                    },
                );
                Ok(value)
            }
        }
    }
}

pub(crate) struct Cache<V> {
    inner: CacheWithKey<(), V>,
}

impl<V> Cache<V>
where
    V: Clone,
{
    pub(crate) fn new(timeout: Duration) -> Self {
        Cache {
            inner: CacheWithKey::new(timeout),
        }
    }

    pub(crate) async fn get_or_update<F, Fut, Err>(&self, f: F) -> Result<V, Err>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<V, Err>>,
    {
        self.inner.get_or_update((), |_| f()).await
    }
}
