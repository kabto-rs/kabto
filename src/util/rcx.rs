use crate::{JSResult, JsCast};
use std::rc::{Rc, Weak};
use std::cell::UnsafeCell;


pub(crate) struct RcX<T>(Rc<UnsafeCell<T>>);

pub(crate) struct WeakX<T>(Weak<UnsafeCell<T>>);

/// SAFETY: single thread
const _: () = {
    unsafe impl<T: Send> Send for RcX<T> {}
    unsafe impl<T: Sync> Sync for RcX<T> {}

    unsafe impl<T: Send> Send for WeakX<T> {}
    unsafe impl<T: Sync> Sync for WeakX<T> {}
};

impl<T> RcX<T> {
    pub(crate) fn new(data: T) -> Self {
        Self(Rc::new(UnsafeCell::new(data)))
    }

    pub(crate) fn downgrade(&self) -> WeakX<T> {
        WeakX(Rc::downgrade(&self.0))
    }

    pub(crate) fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
}
impl<T> std::ops::Deref for RcX<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.0.get()}
    }
}
impl<T> std::ops::DerefMut for RcX<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: single thread
        unsafe {&mut *self.0.get()}
    }
}
impl<T> Clone for RcX<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T> WeakX<T> {
    pub(crate) fn upgrade(&self) -> JSResult<RcX<T>> {
        match self.0.upgrade() {
            Some(rc) => Ok(RcX(rc)),
            None     => Err(::web_sys::Text::new_with_data("invalid `Weak`")?.unchecked_into())
        }
    }
}
impl<T> Clone for WeakX<T> {
    fn clone(&self) -> Self {
        Self(Weak::clone(&self.0))
    }
}

#[cfg(debug_assertions)]
const _: () = {
    use std::fmt::Debug;

    impl<T: Debug> Debug for RcX<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&**self, f)
        }
    }
};
