use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, LockResult};

/*
pub struct Storage<T: Send + Clone> {
    id: u64,
    value: RwLock<T>,
    current: T,
}

impl<T: Send + Clone> Storage<T> {
    fn new(initial_value: T) -> Self {
        Storage{
            id: 0,
            value: RwLock::new(initial_value),
            current: initial_value.clone(),
        }
    }
}

struct State<T: Send + Clone> {
    inner: Arc<Storage<T>>,
}

impl<T: Send + Clone> State<T> {
    pub fn new(initial_value: T) -> Self{
        State{
            inner: Arc::new(Storage::new(initial_value)),
        }
    }
    pub fn update(&self, f: impl FnMut(&T) -> T){
        match self.inner.value.write() {

        }
    }
}

impl<T: Send> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> LockResult<RwLockReadGuard<T>> {

        self.inner.value.read()
    }
}*/