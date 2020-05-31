use std::rc::{Weak};
use std::sync::{RwLock, Arc};

use rand::Rng;
use crate::process::send_command;
use crate::process::command::EngineCommand;
use crate::state::registrar::REGISTRAR;

pub mod registrar;

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct StorageID(u64);

impl StorageID {
    pub fn next() -> Self {
        StorageID(rand::thread_rng().gen())
    }
}

pub struct Storage<T: Send + Clone + PartialEq> {
    id: StorageID,
    value: T,
    update: Vec<Box<dyn FnMut(T) -> T + 'static>>,
}

impl<T: Send + Clone + PartialEq> Storage<T> {
    fn new(value: T) -> Self {
        Storage {
            id: StorageID::next(),
            value,
            update: Vec::new(),
        }
    }
}

pub struct StateHandle(Weak<dyn DynState>);

impl StateHandle {
    pub fn sync(&self) -> bool {
        if let Some(handle) = self.0.upgrade() {
            handle.sync()
        } else {
            false
        }
    }
}

pub trait DynState {
    /// aplly all update function which are send
    /// returns if the new value is different from the old one
    fn sync(&self) -> bool;
}

impl<T: Clone + Send + PartialEq> DynState for RwLock<Storage<T>> {
    fn sync(&self) -> bool {
        let mut storage = self.write().unwrap();

        let init = storage.value.clone();
        let new = storage.update.drain(..)
            .fold(init, |val, mut update| update(val));

        if new.ne(&storage.value) {
            storage.value = new;
            println!("Updated State {:?}", storage.id);
            return true;
        }
        return false;
    }
}

pub struct State<T: Send + Clone + PartialEq> {
    inner: Arc<RwLock<Storage<T>>>,
}

impl<T: Send + Clone + PartialEq> State<T> {
    pub fn new(initial_value: T) -> Self {
        State {
            inner: Arc::new(RwLock::new(Storage::new(initial_value.clone()))),
        }
    }
    pub fn update(&self, f: impl FnMut(T) -> T + 'static) {
        match self.inner.write() {
            Ok(mut lock) => {
                lock.update.push(Box::new(f));
                send_command(EngineCommand::StateChange(lock.id));
            }
            Err(_) => {
                //Its nearly imposible not to succed, since the lock is only acquired for short periods of time,
                //although this implementation is not optimal
                panic!("cant acquire the lock!")
            }
        }
    }

    pub fn load(&mut self) -> T {
        match self.inner.read() {
            Ok(lock) => {
                REGISTRAR.with(|reg|reg.add_used(lock.id));
                lock.value.clone()
            }
            Err(_) => {
                //Its nearly imposible not to succed, since the lock is only acquired for short periods of time,
                //although this implementation is not optimal
                panic!("cant acquire the lock!")
            }
        }
    }
}

pub fn state<T: Clone + Send + PartialEq>(value: T) -> State<T>{
    State::new(value)
}