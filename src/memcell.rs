use core::cell::UnsafeCell;

pub struct MemCell<T: Send + Sync> {
    inner: UnsafeCell<T>,
}

impl<T: Send + Sync> MemCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> &T {
        unsafe { self.inner.get().as_ref().unwrap() }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { self.inner.get().as_mut().unwrap() }
    }
}
