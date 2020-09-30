use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

/// Rc(s) allow you to have multiple references to a thing (stored on the heap). That thing is
/// only deallocated when the last reference to it goes away.
/// The thing being stored needs to be on the heap because
/// Rc(s) are not thread-safe, they are not Sync and not Send
/// Rc(s) will not handle cyclical references, they can cause Rc(s) to never be de-allocated


// RcInner will be shared amongst all clones of an Rc
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl <T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl <T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine
        & unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc {
            // SAFETY: Box does not give us a null pointer
            inner: unsafe { NonNull::new_unchecked( Box::into_raw(inner) ) },
            _marker: PhantomData,
        }
    }
}


impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // SAFETY we are the only Rc left, and we are now being dropped. Therefore, after us,
            // there will be no Rc's and no references to T
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr() ) };
        } else {
            // there are other RCs so just decrease the refcount. DON'T drop the Box
            inner.refcount.set(c - 1);
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn bad_drop_check() {

    }
}