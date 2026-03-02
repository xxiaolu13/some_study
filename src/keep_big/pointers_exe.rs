// 1. cell  new set get

// 3. RefCell
//
use std::cell::Cell;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub struct Xcell<T> {
    value: UnsafeCell<T>,
}

impl<T> Xcell<T> {
    pub fn new(t: T) -> Self {
        Xcell {
            value: UnsafeCell::new(t),
        }
    }
    pub fn set(&self, t: T) {
        unsafe {
            *self.value.get() = t;
        }
    }
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}
// Cell Done

// 2. Rc   deref clone drop
//
pub struct Xrc<T> {
    inner: NonNull<Xinner<T>>,
    _t: PhantomData<T>,
}
pub struct Xinner<T> {
    value: T,
    refcount: Cell<usize>,
}
impl<T> Xrc<T> {
    pub fn new(t: T) -> Self {
        let inner = Xinner {
            value: t,
            refcount: Cell::new(1),
        };
        Xrc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(inner))) },
            _t: PhantomData,
        }
    }
}
impl<T> Drop for Xrc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.refcount.get();
        match count {
            1 => {
                let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
            }
            _ => inner.refcount.set(count - 1),
        }
    }
}
impl<T> Clone for Xrc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Xrc {
            inner: self.inner,
            _t: PhantomData,
        }
    }
}
impl<T> Deref for Xrc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFEYT: because box::into_raw
        &unsafe { self.inner.as_ref() }.value
    }
}
