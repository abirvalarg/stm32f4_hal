use crate::malloc::*;
use core::ptr::NonNull;
use core::{mem::size_of, ops::Deref};
use core::ops::{DerefMut, CoerceUnsized};
use core::marker::Unsize;

pub struct Box<T: ?Sized>(NonNull<T>);

impl<T> Box<T> {
    pub fn new(val: T) -> Box<T> {
        unsafe {
            let ptr = malloc(size_of::<T>()) as *mut T;
            *ptr = val;
            Box(NonNull::new(ptr).unwrap())
        }
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Box<U>> for Box<T> {}

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.0.as_ptr() as *mut ());
        }
    }
}
