use core::{cell::UnsafeCell, ops::{Deref, DerefMut}};

extern "C" {
    fn mask_irq();
    fn unmask_irq();
}

pub fn block_irq<F: Fn()>(func: F) {
    unsafe {
        mask_irq();
        func();
        unmask_irq();
    }
}

pub struct Mutex<T>(UnsafeCell<T>);
pub struct MutexGuard<T>(*mut T);

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Mutex(UnsafeCell::new(value))
    }

    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            mask_irq();
            MutexGuard(self.0.get())
        }
    }
}

impl<T> Deref for Mutex<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0.get()
        }
    }
}

impl<T> Deref for MutexGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.0
        }
    }
}

impl<T> DerefMut for MutexGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.0
        }
    }
}

impl<T> Drop for MutexGuard<T> {
    fn drop(&mut self) {
        unsafe {
            unmask_irq();
        }
    }
}

unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}
