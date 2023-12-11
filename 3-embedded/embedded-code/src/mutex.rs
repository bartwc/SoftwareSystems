/*
mutex.rs - Address Assignment 3 Part 1 - UART

Requirement - Point 2
You must provide a safe abstraction for global mutable state (Mutex)
    - You need to somehow create a way of preventing two threads executing at once.
      Of course there are no threads in the system you are implementing the driver for,
      but concurrency is still an issue.
    - This may need unsafe code, you need to add an explanation on why your implementation is still sound.
*/

// Standard Library for Mutex exist on OS. However, Custom Mutex written for Cortex_M with no OS.

use core::cell::UnsafeCell;
use cortex_m::interrupt::free;

pub struct Mutex<T> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Make a new Mutex. UnsafeCell is a cell type that provides interior mutability through
    /// unsafe operations. It means any data inside an UnsafeCell can be mutated at any time.
    pub const fn new(data: T) -> Self {
        Mutex { data: UnsafeCell::new(data) }
    }

    /// "locks" the mutex, and runs v as a critical section
    /// impl FnOnce(&mut T) -> U: This method signature denotes a function v that takes
    /// a mutable reference to a type T and returns a value of type U.

    /// free(|_| {...}): This function is often used in the context of embedded systems
    /// programming with Rust. It accepts a closure. Inside this function, interrupts
    /// are typically disabled, ensuring mutual exclusion - preventing race conditions
    /// and ensuring the integrity of data during concurrent operations.

    /// unsafe { self.data.get().as_mut().unwrap() }: This code is making use of Rust's
    /// Unsafe functionality to perform an operation which the Rust compiler cannot
    /// guarantee safety. get() function is assumed to get a raw pointer to the data,
    /// as_mut() attempts to convert that pointer to a mutable reference, and unwrap()
    /// removes any possible Option or Result wrapping around that reference,
    /// assuming the value is not None or an Err.
    pub fn update<U>(&self, v: impl FnOnce(&mut T) -> U) -> U {
        free(|_| {
            // The soundness of the code is ensured by mutual exclusion.
            // As the interrupt is turned off, no context switching will happen, and
            // the data would not be edited by multiple parties.
            v(unsafe { self.data.get().as_mut().unwrap() })
        })
    }
}
