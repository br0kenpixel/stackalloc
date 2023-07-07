use core::{cell::UnsafeCell, ops::Deref};

pub struct MemCell<T: Send + Sync> {
    inner: UnsafeCell<T>,
}

impl<T: Send + Sync> MemCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { self.inner.get().as_mut().unwrap() }
    }
}

impl<T: Send + Sync> Deref for MemCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.get().as_ref().unwrap() }
    }
}
