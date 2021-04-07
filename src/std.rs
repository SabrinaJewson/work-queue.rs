use std::cell::UnsafeCell as StdUnsafeCell;

use concurrent_queue::ConcurrentQueue;

pub use std::sync::atomic;
pub use std::sync::Arc;

pub struct UnsafeCell<T>(StdUnsafeCell<T>);

impl<T> UnsafeCell<T> {
    pub fn new(data: T) -> Self {
        Self(StdUnsafeCell::new(data))
    }
    pub fn with<F: FnOnce(*const T) -> R, R>(&self, f: F) -> R {
        f(self.0.get())
    }
    pub fn with_mut<F: FnOnce(*mut T) -> R, R>(&self, f: F) -> R {
        f(self.0.get())
    }
}

pub unsafe fn atomic_u16_unsync_load(atomic: &atomic::AtomicU16) -> u16 {
    *(atomic as *const atomic::AtomicU16).cast()
}

#[derive(Debug)]
pub struct GlobalQueue<T>(ConcurrentQueue<T>);

impl<T> GlobalQueue<T> {
    pub fn new() -> Self {
        Self(ConcurrentQueue::unbounded())
    }
    pub fn push(&self, value: T) {
        let res = self.0.push(value);
        debug_assert!(!res.is_err());
    }
    pub fn pop(&self) -> Option<T> {
        self.0.pop().ok()
    }
}
