use crate::state::StorageID;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::replace;

pub struct StateUseRegistrar {
    used: UnsafeCell<Option<Vec<StorageID>>>,
    non_sync: PhantomData<&'static mut u8>,
}

impl StateUseRegistrar {
    pub fn new() -> Self{
        StateUseRegistrar{
            used: UnsafeCell::new(None),
            non_sync: PhantomData,
        }
    }
    /// returns which states were used during the call of range
    /// this is usfull to which widgets depend on which states
    pub fn used_states(&self, range: impl FnOnce()) -> Vec<StorageID> {
        //inserting a new counter
        let old = unsafe{self.swap(Some(Vec::new()))};

        //execute the function and record usages of States
        range();

        //reseting to the old value
        unsafe{self.swap(old)}.unwrap()
    }
    pub fn add_used(&self, id: StorageID){
        if let Some(used) = unsafe{&mut *self.used.get()} {
            if !used.contains(&id) {
                used.push(id);
            }
        }
    }
    unsafe fn swap(&self, data: Option<Vec<StorageID>>) -> Option<Vec<StorageID>> {
        replace(self.used.get(), data)
    }
}



thread_local! {
    pub static REGISTRAR: StateUseRegistrar = StateUseRegistrar::new();
}