use std::{
    any::Any,
    collections::HashMap,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicI64, Ordering},
    },
};

use parking_lot::RwLock;

use crate::error::JniError;

type ErasedHandle = Arc<dyn Any + Send + Sync>;

static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);
static HANDLES: LazyLock<RwLock<HashMap<i64, ErasedHandle>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) fn insert<T>(value: T) -> i64
where
    T: Any + Send + Sync,
{
    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    HANDLES.write().insert(handle, Arc::new(value));
    handle
}

pub(crate) fn get<T>(handle: i64) -> Result<Arc<T>, JniError>
where
    T: Any + Send + Sync,
{
    let value = HANDLES
        .read()
        .get(&handle)
        .cloned()
        .ok_or_else(|| JniError::InvalidHandle(handle))?;

    value
        .downcast::<T>()
        .map_err(|_| JniError::InvalidHandle(handle))
}

pub(crate) fn remove(handle: i64) {
    HANDLES.write().remove(&handle);
}

pub(crate) fn update<T>(handle: i64, update: impl FnOnce(&mut T)) -> Result<(), JniError>
where
    T: Any + Clone + Send + Sync,
{
    let mut handles = HANDLES.write();
    let current = handles
        .get(&handle)
        .ok_or_else(|| JniError::InvalidHandle(handle))?;
    let mut value = current
        .downcast_ref::<T>()
        .cloned()
        .ok_or_else(|| JniError::InvalidHandle(handle))?;
    update(&mut value);
    handles.insert(handle, Arc::new(value));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Barrier;

    use super::*;

    #[test]
    fn removing_a_handle_is_idempotent() {
        let handle = insert(String::from("value"));

        remove(handle);
        remove(handle);

        assert!(matches!(get::<String>(handle), Err(JniError::InvalidHandle(id)) if id == handle));
    }

    #[test]
    fn an_acquired_value_outlives_its_handle() {
        let handle = insert(String::from("value"));
        let value = get::<String>(handle).unwrap();

        remove(handle);

        assert_eq!(&*value, "value");
        assert!(get::<String>(handle).is_err());
    }

    #[test]
    fn an_acquired_value_survives_concurrent_removal() {
        let handle = insert(String::from("value"));
        let acquired = Arc::new(Barrier::new(2));
        let removed = Arc::new(Barrier::new(2));

        std::thread::scope(|scope| {
            let worker_acquired = acquired.clone();
            let worker_removed = removed.clone();
            scope.spawn(move || {
                let value = get::<String>(handle).unwrap();
                worker_acquired.wait();
                worker_removed.wait();
                assert_eq!(&*value, "value");
            });

            acquired.wait();
            remove(handle);
            removed.wait();
        });

        assert!(get::<String>(handle).is_err());
    }
}
