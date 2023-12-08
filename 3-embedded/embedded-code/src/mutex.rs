use core::cell::UnsafeCell;
use cortex_m::interrupt::free;

pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    // make a new mutex
    pub const fn new(data: T) -> Self {
        Mutex { data: UnsafeCell::new(data) }
    }

    // "locks" the mutex, and runs v as a critical section
    pub fn update<U>(&self, v: impl FnOnce(&mut T) -> U) -> U {
        free(|_| {
            // The soundness of the code is ensured by mutual exclusion.
            // As the interrupt is turned off, no context switching will happen, and
            // the data would not be edited by multiple parties.
            v(unsafe { self.data.get().as_mut().unwrap() })
        })
    }
}
