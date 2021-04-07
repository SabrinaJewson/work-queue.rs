use std::collections::VecDeque;

use loom::sync::Mutex;

pub use loom::cell::UnsafeCell;
pub use loom::sync::atomic;
pub use loom::sync::Arc;

pub unsafe fn atomic_u16_unsync_load(atomic: &atomic::AtomicU16) -> u16 {
    atomic.unsync_load()
}

pub use loom::model as enter;

#[derive(Debug)]
pub struct GlobalQueue<T>(Mutex<VecDeque<T>>);

impl<T> GlobalQueue<T> {
    pub fn new() -> Self {
        Self(Mutex::new(VecDeque::new()))
    }
    pub fn push(&self, value: T) {
        self.0.lock().unwrap().push_back(value);
    }
    pub fn pop(&self) -> Option<T> {
        self.0.lock().unwrap().pop_front()
    }
}
