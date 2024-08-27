use crate::{JSResult, JsCast};
use std::rc::{Rc, Weak};
use std::cell::UnsafeCell;


pub(crate) struct RcX<T>(Rc<UnsafeCell<T>>);

pub(crate) struct WeakX<T>(Weak<UnsafeCell<T>>);

impl<T> RcX<T> {
    pub(crate) fn new(data: T) -> Self {
        Self(Rc::new(UnsafeCell::new(data)))
    }

    pub(crate) fn downgrade(&self) -> WeakX<T> {
        WeakX(Rc::downgrade(&self.0))
    }

    pub(crate) fn as_mut(&self) -> &mut T {
        // SAFETY: single thread
        unsafe {&mut *self.0.get()}
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
            None     => Err(::web_sys::Text::new_with_data("invalid WeakX")?.unchecked_into())
        }
    }
}
